use std::collections::HashMap;

use bridge::message;
use wasmtime::Module;

mod controller;
mod core;
mod service;

fn main() -> anyhow::Result<()> {
    println!("begin");
    let mut instance = core::vm::Instance::new(|engine| -> anyhow::Result<Module> {
        let m = Module::from_file(&engine, "/Users/cattchen/Codes/github.com/ChenKS12138/wasm-lambda/target/wasm32-wasi/debug/hello-world.wasi.wasm")?;
        Ok(m)
    })?;
    println!("instance created");
    let evt = message::TriggerEvent::EventHttpRequest(message::Request {
        path: "www.baidu.com".to_string(),
        headers: HashMap::new(),
        method: "GET".to_string(),
        body: None,
    });
    let resp = instance.run(evt)?;
    println!("{:?}", resp);
    println!("done");
    Ok(())
}
