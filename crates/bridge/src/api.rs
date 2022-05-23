#![allow(dead_code)]

use crate::message;
use bson;

pub type Result<T> = anyhow::Result<T>;

/**
 * hostcall, pass BSON ptr
 * if ptr eq null, no data consumed
 */
mod hostcall {
    #[link(wasm_import_module = "wasm-lambda-bridge")]
    extern "C" {
        pub fn event_recv(ptr: *mut u8, len: u64) -> u64;
        pub fn event_reply(ptr: *const u8, len: u64) -> u64;

        pub fn http_fetch_send(ptr: *const u8, len: u64) -> u64;
        pub fn http_fetch_recv(ptr: *mut u8, len: u64) -> u64;

        pub fn module_call_send(
            module_name_ptr: *const u8,
            module_name_len: u64,
            ptr: *const u8,
            len: u64,
        ) -> u64;
        pub fn module_call_recv(ptr: *mut u8, len: u64) -> u64;
    }
}

pub fn event_recv() -> Result<message::TriggerEvent> {
    unsafe {
        let size = hostcall::event_recv(std::ptr::null_mut(), 0) as usize;
        let mut data = Box::new(vec![0; size]);
        hostcall::event_recv(data.as_mut_ptr(), size as u64);
        let evt = bson::from_slice::<message::TriggerEvent>(&data[..size]).unwrap();
        Ok(evt)
    }
}

pub fn event_reply(reply: message::Response) -> Result<()> {
    let data = bson::to_vec(&reply).unwrap();
    unsafe {
        hostcall::event_reply(data.as_ptr(), data.len() as u64);
    }
    Ok(())
}

pub fn http_fetch(req: message::Request) -> Result<message::Response> {
    let data = bson::to_vec(&req).unwrap();
    unsafe {
        hostcall::http_fetch_send(data.as_ptr(), data.len() as u64);
    }
    unsafe {
        let size = hostcall::http_fetch_recv(std::ptr::null_mut(), 0) as usize;
        let mut data = Box::new(vec![0; size]);
        hostcall::http_fetch_recv(data.as_mut_ptr(), size as u64);
        let resp = bson::from_slice::<message::Response>(&data[..size]).unwrap();
        Ok(resp)
    }
}

pub fn module_call(module_name: String, req: message::Request) -> Result<message::Response> {
    let data = bson::to_vec(&req).unwrap();
    unsafe {
        hostcall::module_call_send(
            module_name.as_ptr(),
            module_name.len() as u64,
            data.as_ptr(),
            data.len() as u64,
        );
    }
    unsafe {
        let size = hostcall::module_call_recv(std::ptr::null_mut(), 0) as usize;
        let mut data = Box::new(vec![0; size]);
        hostcall::module_call_recv(data.as_mut_ptr(), size as u64);
        let resp = bson::from_slice::<message::Response>(&data[..size]).unwrap();
        Ok(resp)
    }
}
