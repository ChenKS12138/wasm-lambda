use serde_json::json;
use wasm_lambda_bridge::{
    codegen::{self, get},
    compose_routers,
    core::value::{self, Response},
    dispatch, make_json_response, Result,
};

#[codegen::main]
fn main(event: value::TriggerEvent) -> Result<Response> {
    dispatch!(event, [compose_routers!("/api", [index])])
}

#[get("/")]
fn index() -> Result<Response> {
    Ok(make_json_response!(json!({
        "code":0
    })))
}
