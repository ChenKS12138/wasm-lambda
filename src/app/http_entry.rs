use std::{collections::HashMap, net::SocketAddr};

use bridge::value;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, StatusCode,
};
use wasmtime::Module;

use crate::core;

pub async fn make_serve() -> anyhow::Result<()> {
    let addr: SocketAddr = "0.0.0.0:3333".parse()?;

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(entry)) });

    Server::bind(&addr).serve(service).await?;
    Ok(())
}

async fn entry(req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let path = req.uri().path_and_query().unwrap().to_string();
    let mut instance = core::vm::Instance::new(|engine| -> anyhow::Result<Module> {
        let m = Module::from_file(&engine, "/home/cattchen/codes/github.com/ChenKS12138/wasm-lambda/target/wasm32-wasi/debug/hello-world.wasi.wasm")?;
        Ok(m)
    })?;
    let mut headers = HashMap::new();
    for (key, value) in req.headers() {
        headers.insert(
            key.as_str().to_string(),
            value.to_str().unwrap().to_string(),
        );
    }
    let method = req.method().to_string();
    let body = hyper::body::to_bytes(req.into_body()).await?;
    let whole_body = body.to_vec();
    let evt = value::TriggerEvent::EventHttpRequest(value::Request {
        path,
        headers,
        method,
        body: Some(whole_body),
    });
    let evt_response = instance.run(evt).await?;
    let response = match evt_response {
        None => Response::new(Body::empty()),
        Some(evt_response) => {
            let response =
                Response::builder().status(StatusCode::from_u16(evt_response.status as u16)?);

            // for (key, value) in &evt_response.headers {
            //     response.header(key, value);
            // }

            let response = response.body(match evt_response.body {
                None => Body::empty(),
                Some(body) => Body::from(body),
            })?;
            response
        }
    };
    Ok(response)
}
