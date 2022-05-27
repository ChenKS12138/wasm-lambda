use std::{collections::HashMap, env};

use bridge::value;

fn main() -> bridge::api::Result<()> {
    let event = bridge::api::event_recv()?;
    // println!("{:?}", event);
    let response = bridge::value::Response {
        status: 200,
        headers: HashMap::new(),
        body: Some("hello world\n".try_into()?),
    };
    bridge::api::event_reply(response)?;
    // let request = value::Request {
    //     path: "https://icanhazip.com".to_string(),
    //     headers: HashMap::new(),
    //     method: "GET".to_string(),
    //     body: None,
    // };
    // let response = bridge::api::http_fetch(request)?;
    // println!("{:?}", response);
    // println!("Hello, world!");
    Ok(())
}
