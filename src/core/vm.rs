use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
};

use bridge::message;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::{add_to_linker, WasiCtx, WasiCtxBuilder};

use super::hostcall::register;

pub type InstanceIOBuffer = Arc<(
    Mutex<LinkedList<message::TriggerEvent>>,
    Mutex<LinkedList<message::Response>>,
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
    // engine: Engine,
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
        let mut engine = Engine::default();
        let data = InstanceState::new(wasi_ctx);
        let mut linker = Linker::new(&engine);
        let store = Store::new(&engine, data);
        let module = load_module(&mut engine)?;
        let io_buffer = Arc::new((Mutex::new(LinkedList::new()), Mutex::new(LinkedList::new())));
        register(&mut linker, io_buffer.clone())?;
        add_to_linker(&mut linker, |ctx: &mut InstanceState| &mut ctx.wasi)?;
        Ok(Self {
            // engine,
            linker,
            store,
            module,
            io_buffer,
        })
    }
    pub fn run(
        &mut self,
        event: message::TriggerEvent,
    ) -> anyhow::Result<Option<message::Response>> {
        let instance = self.linker.instantiate(&mut self.store, &self.module)?;
        self.io_buffer.0.lock().unwrap().push_back(event);
        let func = instance.get_typed_func(&mut self.store, "_start")?;
        func.call(&mut self.store, ())?;
        Ok(self.io_buffer.1.lock().unwrap().pop_front())
    }
}
