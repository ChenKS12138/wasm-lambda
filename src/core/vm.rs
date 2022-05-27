use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
};

use bridge::value;
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::{add_to_linker, WasiCtx, WasiCtxBuilder};

use super::hostcall::register;

pub type InstanceIOBuffer = Arc<(
    Mutex<LinkedList<value::TriggerEvent>>,
    Mutex<LinkedList<value::Response>>,
)>;

pub struct InstanceState {
    wasi: WasiCtx,
}

impl InstanceState {
    fn new(wasi_ctx: WasiCtx) -> Self {
        Self { wasi: wasi_ctx }
    }
}

pub struct Instance {
    linker: Linker<InstanceState>,
    store: Store<InstanceState>,
    module: Module,
    io_buffer: InstanceIOBuffer,
}

impl Instance {
    pub fn new<F>(load_module: F) -> anyhow::Result<Self>
    where
        F: Fn(&mut Engine) -> anyhow::Result<Module>,
    {
        let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
        let data = InstanceState::new(wasi_ctx);

        let mut engine = Engine::new(Config::new().async_support(true)).unwrap();
        let mut linker = Linker::new(&engine);
        let store = Store::new(&engine, data);

        let module = load_module(&mut engine)?;
        let io_buffer = Arc::new((Mutex::new(LinkedList::new()), Mutex::new(LinkedList::new())));
        register(&mut linker, io_buffer.clone())?;
        add_to_linker(&mut linker, |ctx: &mut InstanceState| &mut ctx.wasi)?;
        Ok(Self {
            linker,
            store,
            module,
            io_buffer,
        })
    }
    pub async fn run(
        &mut self,
        event: value::TriggerEvent,
    ) -> anyhow::Result<Option<value::Response>> {
        let instance = self
            .linker
            .instantiate_async(&mut self.store, &self.module)
            .await?;
        self.io_buffer.0.lock().unwrap().push_back(event);
        let func = instance.get_typed_func(&mut self.store, "_start")?;
        func.call_async(&mut self.store, ()).await?;
        Ok(self.io_buffer.1.lock().unwrap().pop_front())
    }
}
