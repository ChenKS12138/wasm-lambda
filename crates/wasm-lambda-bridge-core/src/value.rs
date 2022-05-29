use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ModuleName = String;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Request {
    pub path: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub status: u64,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TriggerEvent {
    EventHttpRequest(Request),
    EventInternalModuleCall(ModuleName, Request),
    // EventCronTask(Request),
    // EventManualTask(Request),
}
