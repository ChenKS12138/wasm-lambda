use std::{collections::HashMap, sync::Arc};

use super::vm::{Environment, InstanceState};
use bridge::{value, WASM_MODULE_NAME};
use wasmtime::{Caller, Extern, Linker};

pub fn add_to_linker(
    linker: &mut Linker<InstanceState>,
    environment: Arc<Environment>,
) -> anyhow::Result<()> {
    linker.func_wrap(
        WASM_MODULE_NAME,
        "event_recv",
        move |mut caller: Caller<'_, InstanceState>, ptr: i32, len: u64| -> u64 {
            if let Some(event) = {
                let mut io_buffer_0 = caller.data().io_buffer.0.lock().unwrap();
                io_buffer_0.pop_front()
            } {
                let event_data = bson::to_vec(&event).unwrap();
                if event_data.len() > len as usize || ptr == 0 {
                    let mut io_buffer_0 = caller.data().io_buffer.0.lock().unwrap();
                    io_buffer_0.push_back(event);
                    return event_data.len() as u64;
                }
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => {
                        panic!("memory not found");
                    }
                };
                let data = mem.data_mut(&mut caller).get_mut(ptr as usize..).unwrap();
                unsafe {
                    std::ptr::copy(event_data.as_ptr(), data.as_mut_ptr(), len as usize);
                }
                event_data.len() as u64
            } else {
                return 0;
            }
        },
    )?;

    linker.func_wrap(
        WASM_MODULE_NAME,
        "event_reply",
        move |mut caller: Caller<'_, InstanceState>, ptr: i32, len: u64| -> u64 {
            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => {
                    panic!("memory not found");
                }
            };
            let response_data = mem
                .data(&mut caller)
                .get(ptr as usize..(ptr as usize + len as usize))
                .unwrap();
            let response = bson::from_slice::<value::Response>(&response_data).unwrap();
            let mut responses = caller.data().io_buffer.1.lock().unwrap();
            responses.push_back(response);
            len
        },
    )?;

    linker.func_wrap2_async(
        WASM_MODULE_NAME,
        "http_fetch_send",
        move |mut caller, ptr: i32, len: u64| {
            let io_buffer_clone = caller.data().io_buffer.clone();
            Box::new(async move {
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => {
                        panic!("memory not found");
                    }
                };
                let request_data = mem
                    .data(&mut caller)
                    .get(ptr as usize..(ptr as usize + len as usize))
                    .unwrap();
                let request_data = bson::from_slice::<value::Request>(&request_data).unwrap();

                let client = hyper::Client::new();
                let mut request = hyper::Request::builder()
                    .method(hyper::Method::from_bytes(request_data.method.as_bytes()).unwrap())
                    .uri(&request_data.path);
                let headers = request.headers_mut().unwrap();
                for header in request_data.headers.iter() {
                    headers.insert(
                        hyper::header::HeaderName::from_bytes(header.0.as_bytes()).unwrap(),
                        hyper::header::HeaderValue::from_str(header.1.as_str()).unwrap(),
                    );
                }
                let request = request
                    .body(hyper::Body::from(request_data.body.unwrap_or_default()))
                    .unwrap();
                let response = client.request(request).await.unwrap();
                let status = response.status().as_u16() as u64;
                let headers: HashMap<String, String> = response
                    .headers()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
                    .collect();
                let body = hyper::body::to_bytes(response.into_body())
                    .await
                    .unwrap()
                    .to_vec();
                let response_data = value::Response {
                    status,
                    headers: headers,
                    body: Some(body),
                };
                let mut fetch_result = io_buffer_clone.2.lock().unwrap();
                fetch_result.push_back(response_data);
                len
            })
        },
    )?;

    linker.func_wrap(
        WASM_MODULE_NAME,
        "http_fetch_recv",
        move |mut caller: Caller<'_, InstanceState>, ptr: i32, len: u64| -> u64 {
            if let Some(response) = {
                let mut io_buffer_2 = caller.data().io_buffer.2.lock().unwrap();
                io_buffer_2.pop_front()
            } {
                let response_data = bson::to_vec(&response).unwrap();
                if response_data.len() > len as usize || ptr == 0 {
                    let mut io_buffer_2 = caller.data().io_buffer.2.lock().unwrap();
                    io_buffer_2.push_back(response);
                    return response_data.len() as u64;
                }
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => {
                        panic!("memory not found");
                    }
                };
                let data = mem.data_mut(&mut caller).get_mut(ptr as usize..).unwrap();
                unsafe {
                    std::ptr::copy(response_data.as_ptr(), data.as_mut_ptr(), len as usize);
                }
                response_data.len() as u64
            } else {
                return 0;
            }
        },
    )?;
    linker.func_wrap4_async(
        WASM_MODULE_NAME,
        "module_call_send",
        move |mut caller, module_name_ptr: i32, module_name_len: u64, ptr: i32, len: u64| {
            let environment = environment.clone();
            let io_buffer_clone = caller.data().io_buffer.clone();
            let current_module_name = caller.data().module_name.clone();
            Box::new(async move {
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => {
                        panic!("memory not found");
                    }
                };
                let module_name_data = mem
                    .data(&mut caller)
                    .get(
                        module_name_ptr as usize
                            ..(module_name_ptr as usize + module_name_len as usize),
                    )
                    .unwrap();
                let module_name = String::from_utf8(module_name_data.to_vec()).unwrap();

                let request_data = mem
                    .data(&mut caller)
                    .get(ptr as usize..(ptr as usize + len as usize))
                    .unwrap();
                let request_data = bson::from_slice::<value::Request>(&request_data).unwrap();
                let response = environment
                    .call(
                        module_name,
                        "latest".to_string(),
                        bridge::value::TriggerEvent::EventInternalModuleCall(
                            current_module_name,
                            request_data,
                        ),
                    )
                    .await
                    .unwrap();
                if let Some(response) = response.0 {
                    io_buffer_clone.2.lock().unwrap().push_back(response);
                }
                len
            })
        },
    )?;
    linker.func_wrap(
        WASM_MODULE_NAME,
        "module_call_recv",
        move |mut caller: Caller<'_, InstanceState>, ptr: i32, len: u64| -> u64 {
            if let Some(response) = {
                let mut io_buffer_2 = caller.data().io_buffer.2.lock().unwrap();
                io_buffer_2.pop_front()
            } {
                let response_data = bson::to_vec(&response).unwrap();
                if response_data.len() > len as usize || ptr == 0 {
                    let mut io_buffer_2 = caller.data().io_buffer.2.lock().unwrap();
                    io_buffer_2.push_back(response);
                    return response_data.len() as u64;
                }
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => {
                        panic!("memory not found");
                    }
                };
                let data = mem.data_mut(&mut caller).get_mut(ptr as usize..).unwrap();
                unsafe {
                    std::ptr::copy(response_data.as_ptr(), data.as_mut_ptr(), len as usize);
                }
                response_data.len() as u64
            } else {
                0
            }
        },
    )?;
    Ok(())
}
