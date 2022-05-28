use std::{collections::HashMap, env};

use bridge::value;

fn main() -> bridge::api::Result<()> {
    let event = bridge::api::event_recv()?;

    // if let bridge::value::TriggerEvent::EventInternalModuleCall(source, event) = event {
    //     println!("{:?} {:?}", source, event);
    // } else {
    //     let module_call_event = bridge::value::Request {
    //         path: "/".to_string(),
    //         method: "GET".to_string(),
    //         headers: HashMap::default(),
    //         body: None,
    //     };
    //     let module_call_response = bridge::api::module_call("kv".to_string(), module_call_event)?;
    //     println!("{:?}", module_call_response);
    // }

    let response = bridge::value::Response {
        status: 200,
        headers: HashMap::new(),
        body: Some("hello world\n".try_into()?),
    };
    bridge::api::event_reply(response).unwrap();
    Ok(())
}

// fn test_http_fetch() {
//     let request = value::Request {
//         path: "https://icanhazip.com".to_string(),
//         headers: HashMap::new(),
//         method: "GET".to_string(),
//         body: None,
//     };
//     let response = bridge::api::http_fetch(request)?;
//     println!("{:?}", response);
// }
