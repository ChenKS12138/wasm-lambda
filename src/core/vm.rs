use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
};

use bridge::value;
use wasmtime::{Config, Engine, Linker, Module as InternalModule, Store};
use wasmtime_wasi::{self, WasiCtx, WasiCtxBuilder};

use super::hostcall;

pub type InstanceIOBuffer = Arc<(
    Mutex<LinkedList<value::TriggerEvent>>, // in event
    Mutex<LinkedList<value::Response>>,     // out event
    Mutex<LinkedList<value::Response>>,     // recv http_fetch/module_call
)>;

pub struct InstanceState {
    pub module_name: String,
    pub wasi: WasiCtx,
    pub io_buffer: InstanceIOBuffer,
}

impl InstanceState {
    fn new(module_name: String, wasi_ctx: WasiCtx) -> Self {
        Self {
            module_name,
            wasi: wasi_ctx,
            io_buffer: Arc::new((
                Mutex::new(LinkedList::new()),
                Mutex::new(LinkedList::new()),
                Mutex::new(LinkedList::new()),
            )),
        }
    }
}

#[async_trait::async_trait]
pub trait WasmModuleLoader: Send + Sync {
    async fn fetch_module(
        &self,
        engine: Arc<Engine>,
        module_prefix: String,
    ) -> anyhow::Result<Module>;
}

#[derive(Clone)]
pub struct Module {
    pub module_name: String,
    pub version_digest: String,
    pub module: InternalModule,
    pub envs: Vec<(String, String)>,
}

pub struct Environment {
    pub engine: Arc<Engine>,
    pub linker: Arc<tokio::sync::Mutex<Linker<InstanceState>>>,
    pub module_fetcher: Arc<Box<dyn WasmModuleLoader>>,
}

impl Environment {
    pub async fn new(
        module_fetcher: Box<dyn WasmModuleLoader>,
        engine: Arc<Engine>,
    ) -> anyhow::Result<Arc<Self>> {
        let module_fetcher = Arc::new(module_fetcher);
        let linker = Arc::new(tokio::sync::Mutex::new(Linker::new(&engine)));
        let environment = Arc::new(Self {
            engine,
            linker,
            module_fetcher,
        });
        {
            let mut linker = environment.linker.lock().await;
            wasmtime_wasi::add_to_linker(&mut linker, |ctx: &mut InstanceState| &mut ctx.wasi)?;
            hostcall::add_to_linker(&mut linker, environment.clone())?;
        }
        Ok(environment)
    }
    pub fn new_engine() -> anyhow::Result<Arc<Engine>> {
        Ok(Arc::new(Engine::new(Config::new().async_support(true))?))
    }
    pub async fn call(
        &self,
        module_name: String,
        event: bridge::value::TriggerEvent,
    ) -> anyhow::Result<(Option<value::Response>, String)> {
        let mut module = self
            .module_fetcher
            .fetch_module(self.engine.clone(), module_name.to_string())
            .await?;
        module
            .envs
            .push(("MODULE_NAME".to_string(), module_name.to_string()));
        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdio()
            .envs(&module.envs)?
            .build();
        let data = InstanceState::new(module_name.to_string(), wasi_ctx);
        {
            let mut io_buffer_evt_in = data.io_buffer.0.lock().unwrap();
            io_buffer_evt_in.push_back(event);
        }
        let mut store = Store::new(&self.engine, data);

        let linker_guard = self.linker.lock().await;

        let instance = linker_guard
            .instantiate_async(&mut store, &module.module)
            .await?;
        let start = instance.get_typed_func(&mut store, "_start")?;
        start.call_async(&mut store, ()).await?;
        let mut io_buffer_response = store.data().io_buffer.1.lock().unwrap();
        Ok((
            io_buffer_response.pop_front(),
            module.version_digest.clone(),
        ))
    }
}
