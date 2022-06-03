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
                    compose_routers!(
                        "/api",
                        [index, capitalize_sentence, echo_query, echo_person]
                    )
                ]
            )
        ]
    )
}

#[get("/")]
fn index(event: web::TriggerEvent) -> Result<Response> {
    Ok(make_response!(format!("Hello World!\n {:?}\n", event)))
}

#[get("/capitalize/:sentence")]
fn capitalize_sentence(params: web::Params) -> Result<Response> {
    let sentence = params
        .get("sentence")
        .and_then(|v| Some(v.to_string()))
        .unwrap_or_default();
    let sentence: String = sentence
        .chars()
        .map(|v| v.to_uppercase().nth(0).unwrap_or_default())
        .into_iter()
        .collect();
    Ok(make_response!(format!("Result: {}", sentence)))
}

#[get("/echo-query")]
fn echo_query(query: web::Query) -> Result<Response> {
    Ok(make_response!(format!("Result: {:?}", query)))
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u64,
}

#[post("/echo-person")]
fn echo_person(data: web::Json<Person>) -> Result<Response> {
    Ok(make_json_response!(json!({
        "code": 0,
        "message": "ok",
        "data": *data
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
                "Location"=> "/hello-world/latest/index.html"
            ),
            ""
        ));
    }
    context
}
