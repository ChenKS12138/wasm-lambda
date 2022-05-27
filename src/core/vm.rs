use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
};

use bridge::value;
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::{self, WasiCtx, WasiCtxBuilder};

use super::hostcall;

pub type InstanceIOBuffer = Arc<(
    Mutex<LinkedList<value::TriggerEvent>>, // in event
    Mutex<LinkedList<value::Response>>,     // out event
    Mutex<LinkedList<value::Response>>,     // recv http_fetch
)>;

pub struct InstanceState {
    pub wasi: WasiCtx,
    pub io_buffer: InstanceIOBuffer,
}

impl InstanceState {
    fn new(wasi_ctx: WasiCtx) -> Self {
        Self {
            wasi: wasi_ctx,
            io_buffer: Arc::new((
                Mutex::new(LinkedList::new()),
                Mutex::new(LinkedList::new()),
                Mutex::new(LinkedList::new()),
            )),
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
        module: Module,
        envs: &[(String, String)],
        event: bridge::value::TriggerEvent,
    ) -> anyhow::Result<Option<value::Response>> {
        let wasi_ctx = WasiCtxBuilder::new().envs(envs)?.build();
        let data = InstanceState::new(wasi_ctx);
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
