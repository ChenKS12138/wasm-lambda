use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
};

use bridge::value;
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::{self, WasiCtx, WasiCtxBuilder};

use crate::app::infra::AppState;

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
    pub app_state: AppState,
}

impl InstanceState {
    fn new(module_name: String, wasi_ctx: WasiCtx, app_state: AppState) -> Self {
        Self {
            module_name,
            wasi: wasi_ctx,
            io_buffer: Arc::new((
                Mutex::new(LinkedList::new()),
                Mutex::new(LinkedList::new()),
                Mutex::new(LinkedList::new()),
            )),
            app_state,
        }
    }
}

pub struct Environment {
    pub engine: Engine,
    pub linker: Linker<InstanceState>,
}

impl Environment {
    pub fn new() -> anyhow::Result<Self> {
        let mut engine = Engine::new(Config::new().async_support(true))?;
        let mut linker = Linker::new(&engine);
        hostcall::add_to_linker(&mut linker)?;
        wasmtime_wasi::add_to_linker(&mut linker, |ctx: &mut InstanceState| &mut ctx.wasi)?;
        Ok(Self { engine, linker })
    }
    pub async fn run(
        &self,
        module_name: String,
        app_state: AppState,
        module: Module,
        envs: &[(String, String)],
        event: bridge::value::TriggerEvent,
    ) -> anyhow::Result<Option<value::Response>> {
        let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().envs(envs)?.build();
        let data = InstanceState::new(module_name, wasi_ctx, app_state);
        {
            let mut io_buffer_evt_in = data.io_buffer.0.lock().unwrap();
            io_buffer_evt_in.push_back(event);
        }
        let mut store = Store::new(&self.engine, data);
        let instance = self.linker.instantiate_async(&mut store, &module).await?;
        let start = instance.get_typed_func(&mut store, "_start")?;
        start.call_async(&mut store, ()).await?;
        let mut io_buffer_response = store.data().io_buffer.1.lock().unwrap();
        Ok(io_buffer_response.pop_front())
    }
}
