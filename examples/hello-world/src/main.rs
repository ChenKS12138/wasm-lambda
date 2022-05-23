use std::collections::HashMap;

use bridge::message;

fn main() -> bridge::api::Result<()> {
    // let event = bridge::api::event_recv()?;
    // println!("{:?}", event);
    // let response = bridge::message::Response {
    //     status: 200,
    //     headers: HashMap::new(),
    //     body: None,
    // };
    // bridge::api::event_reply(response)?;
    let request = message::Request {
        path: "https://icanhazip.com".to_string(),
        headers: HashMap::new(),
        method: "GET".to_string(),
        body: None,
    };
    let response = bridge::api::http_fetch(request)?;
    println!("{:?}", response);
    println!("Hello, world!");
    Ok(())
}
