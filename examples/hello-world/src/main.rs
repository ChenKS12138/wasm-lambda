use std::collections::HashMap;

use wasm_lambda_bridge::{
    codegen,
    core::{
        value::{self, Response, TriggerEvent},
        Result,
    },
};

/**
 * make_router!(index,login)(event)
 *
 * #[get("/")]
 * fn index(){
 *
 * }
 *
 */

#[codegen::main]
fn main(_event: TriggerEvent) -> Result<Response> {
    index(_event)
}

fn index(_event: TriggerEvent) -> Result<Response> {
    Ok(value::Response {
        status: 200,
        headers: HashMap::new(),
        body: Some("hello world\n".try_into()?),
    })
}

fn login(_event: TriggerEvent) -> Result<Response> {
    Ok(value::Response {
        status: 200,
        headers: HashMap::new(),
        body: Some("login page.\n".try_into()?),
    })
}
