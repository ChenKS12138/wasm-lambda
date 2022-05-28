#![allow(dead_code)]

use crate::{value, Result};
use bson;

/**
 * hostcall, pass BSON ptr
 * if ptr eq null, no data consumed
 */
mod raw {
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

pub fn event_recv() -> Result<value::TriggerEvent> {
    unsafe {
        let size = raw::event_recv(std::ptr::null_mut(), 0) as usize;
        let mut data = Box::new(vec![0; size]);
        raw::event_recv(data.as_mut_ptr(), size as u64);
        let evt = bson::from_slice::<value::TriggerEvent>(&data[..size]).unwrap();
        Ok(evt)
    }
}

pub fn event_reply(reply: value::Response) -> Result<()> {
    let data = bson::to_vec(&reply).unwrap();
    unsafe {
        raw::event_reply(data.as_ptr(), data.len() as u64);
    }
    Ok(())
}

pub fn http_fetch(req: value::Request) -> Result<value::Response> {
    let data = bson::to_vec(&req).unwrap();
    unsafe {
        raw::http_fetch_send(data.as_ptr(), data.len() as u64);
    }
    unsafe {
        let size = raw::http_fetch_recv(std::ptr::null_mut(), 0) as usize;
        let mut data = Box::new(vec![0; size]);
        raw::http_fetch_recv(data.as_mut_ptr(), size as u64);
        let resp = bson::from_slice::<value::Response>(&data[..size]).unwrap();
        Ok(resp)
    }
}

pub fn module_call(module_name: String, req: value::Request) -> Result<value::Response> {
    let data = bson::to_vec(&req).unwrap();
    unsafe {
        raw::module_call_send(
            module_name.as_ptr(),
            module_name.len() as u64,
            data.as_ptr(),
            data.len() as u64,
        );
    }
    unsafe {
        let size = raw::module_call_recv(std::ptr::null_mut(), 0) as usize;
        let mut data = Box::new(vec![0; size]);
        raw::module_call_recv(data.as_mut_ptr(), size as u64);
        let resp = bson::from_slice::<value::Response>(&data[..size]).unwrap();
        Ok(resp)
    }
}
