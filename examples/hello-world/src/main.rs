use serde::{Deserialize, Serialize};
use serde_json::{self, json};

use wasm_lambda_bridge::{
    codegen::{self, get, post, static_resource},
    core::value::{self, Response},
    make_json_response, make_response, web, Result,
};

#[codegen::main]
fn main(event: value::TriggerEvent) -> Result<Response> {
    // handle_event!(
    //     event,
    //     compose_routes!(
    //         "/",
    //         [
    //             index,
    //             get_user,
    //             create_user,
    //             static_resource!(prefix = "/", folder = "public/")
    //         ]
    //     )
    // )
    make_response!("hello")
}

#[get("/")]
fn index(_query: web::Query, _event: web::TriggerEvent) -> Result<Response> {
    make_response!("123Hello, world!\n")
}

#[get("/user/:user_id")]
fn get_user(_event: web::TriggerEvent, _param: web::Params) -> Result<Response> {
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
fn create_user(_data: web::Json<CreateUserDto>) -> Result<Response> {
    make_json_response!(json!({
        "code": 0,
        "message": "ok",
        "data": true
    }))
}

// #[middleware("/")]
// fn not_found(){
// }
