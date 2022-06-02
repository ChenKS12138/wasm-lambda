use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{self, json};

use wasm_lambda_bridge::{
    codegen::{self, get, middleware, post, static_resource},
    compose_routers,
    core::value::{self, Response},
    dispatch, make_headers, make_json_response, make_response,
    web::{self, MiddlewareContext, MiddlewareNext},
    Result,
};

#[codegen::main]
fn main(event: value::TriggerEvent) -> Result<Response> {
    dispatch!(
        event,
        [
            not_found,
            try_index_html,
            compose_routers!(
                "/",
                [
                    static_resource!(prefix = "/", folder = "public/"),
                    compose_routers!("/api", [index, get_user, create_user])
                ]
            )
        ]
    )
}

#[get("/")]
fn index(query: web::Query, _event: web::TriggerEvent, headers: web::Headers) -> Result<Response> {
    Ok(make_response!(format!(
        "123Hello, world! {:?} {:?}\n",
        query, headers
    )))
}

#[get("/user/:user_id")]
fn get_user(_event: web::TriggerEvent, param: web::Params) -> Result<Response> {
    Ok(make_json_response!(json!({
        "code": 0,
        "message": "ok",
        "data": format!("{:?}",param)
    })))
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateUserDto {
    name: String,
    age: u64,
}

#[post("/user")]
fn create_user(_data: web::Json<CreateUserDto>) -> Result<Response> {
    Ok(make_json_response!(json!({
        "code": 0,
        "message": "ok",
        "data": true
    })))
}

#[middleware]
fn not_found(context: MiddlewareContext, next: MiddlewareNext) -> MiddlewareContext {
    let mut context = next.call(context);
    if context.1.is_none() {
        context.1 = Some(make_response!(404, "404 Not Found"));
    }
    context
}

#[middleware]
fn try_index_html(context: MiddlewareContext, next: MiddlewareNext) -> MiddlewareContext {
    let mut context = next.call(context);
    if context.1.is_none() {
        context.1 = Some(make_response!(
            301,
            make_headers!(
                "Location"=> "/portal/latest/index.html"
            ),
            ""
        ));
    }
    context
}
