use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ModuleName = String;

pub type Params = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Request {
    pub path: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub status: u64,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TriggerEvent {
    EventHttpRequest(Request),
    EventInternalModuleCall(ModuleName, Request),
    // EventCronTask(Request),
    // EventManualTask(Request),
}
