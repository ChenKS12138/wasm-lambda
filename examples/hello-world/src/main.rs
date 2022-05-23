use std::collections::HashMap;

fn main() -> bridge::api::Result<()> {
    let event = bridge::api::event_recv()?;
    println!("{:?}", event);
    let response = bridge::message::Response {
        status: 200,
        headers: HashMap::new(),
        body: None,
    };
    bridge::api::event_reply(response)?;
    println!("Hello, world!");
    Ok(())
}
