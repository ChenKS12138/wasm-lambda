use serde::{Deserialize, Serialize};
use serde_json::{self, json};

use wasm_lambda_bridge::{
    codegen::{self, get, post},
    core::{
        value::{self, Response},
        web, Result,
    },
    dispatch_event, make_json_response, make_response,
};

#[codegen::main]
fn main(event: value::TriggerEvent) -> Result<Response> {
    dispatch_event!(event, [index, get_user, create_user])
}

#[get("/")]
fn index(query: web::Query, event: web::TriggerEvent) -> Result<Response> {
    println!("{:?} {:?}", query, event);
    make_response!("Hello, world!\n")
}

#[get("/user/:user_id")]
fn get_user(event: web::TriggerEvent, param: web::Params) -> Result<Response> {
    println!("{:?} {:?}", param, event);
    make_json_response!(json!({
        "code": 0,
        "message": "ok",
        "data": true
    }))
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateUserDto {
    name: String,
    age: u64,
}

#[post("/user")]
fn create_user(data: web::Json<CreateUserDto>) -> Result<Response> {
    println!("{:?}", data);
    make_json_response!(json!({
        "code": 0,
        "message": "ok",
        "data": true
    }))
}
