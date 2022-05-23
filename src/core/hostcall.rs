use super::vm::{InstanceIOBuffer, InstanceState};
use bridge::message;
use wasmtime::{Caller, Extern, Linker};

const MODULE_NAME: &str = "wasm-lambda-bridge";

pub fn register(
    linker: &mut Linker<InstanceState>,
    io_buffer: InstanceIOBuffer,
) -> anyhow::Result<()> {
    let io_buffer_clone = io_buffer.clone();
    linker.func_wrap(
        MODULE_NAME,
        "event_recv",
        move |mut caller: Caller<'_, InstanceState>, ptr: i32, len: u64| -> u64 {
            let mut events = io_buffer_clone.0.lock().unwrap();
            let event = events.front().unwrap();
            let event_data = bson::to_vec(&event).unwrap();
            if event_data.len() > len as usize || ptr == 0 {
                return event_data.len() as u64;
            }
            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => {
                    println!("memory not found");
                    return 0;
                }
            };
            let data = mem.data_mut(&mut caller).get_mut(ptr as usize..).unwrap();
            unsafe {
                std::ptr::copy(event_data.as_ptr(), data.as_mut_ptr(), len as usize);
            }
            events.pop_front();
            event_data.len() as u64
        },
    )?;

    let io_buffer_clone = io_buffer.clone();
    linker.func_wrap(
        MODULE_NAME,
        "event_reply",
        move |mut caller: Caller<'_, InstanceState>, ptr: i32, len: u64| -> u64 {
            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => {
                    println!("memory not found");
                    return 0;
                }
            };
            let response_data = mem
                .data(&mut caller)
                .get(ptr as usize..(ptr as usize + len as usize))
                .unwrap();
            let response = bson::from_slice::<message::Response>(&response_data).unwrap();
            let mut responses = io_buffer_clone.1.lock().unwrap();
            responses.push_back(response);
            len
        },
    )?;
    Ok(())
}
