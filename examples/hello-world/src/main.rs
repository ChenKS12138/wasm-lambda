use serde::{Deserialize, Serialize};
use serde_json::{self, json};

use wasm_lambda_bridge::{
    codegen::{self, get, post},
    core::{
        value::{Params, Response, TriggerEvent},
        Result,
    },
    dispatch_event, make_headers, make_json_response, make_response,
};

#[derive(Debug, Deserialize, Serialize)]
struct RestfulResult {
    code: u64,
    message: String,
    data: serde_json::Value,
}

#[codegen::main]
fn main(event: TriggerEvent) -> Result<Response> {
    dispatch_event!(event, [index, login, user_info])
}

#[get("/")]
fn index(_event: TriggerEvent, _params: Params) -> Result<Response> {
    make_response!("Hello, world!\n")
}

#[post("/login")]
fn login(_event: TriggerEvent, _params: Params) -> Result<Response> {
    make_response!(
        200,
        make_headers!(
            "Content-Type" => "text/plain",
            "X-FOO" => "bar"
        ),
        "login\n"
    )
}

#[get("/user/info")]
fn user_info(_event: TriggerEvent, _params: Params) -> Result<Response> {
    make_json_response!(json!({
        "code":-1,
        "message":"Unauthorized",
    }))
}
