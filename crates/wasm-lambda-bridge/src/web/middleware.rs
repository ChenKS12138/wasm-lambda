use std::sync::Arc;

pub type MiddlewareContext = (
    (bridge_core::value::TriggerEvent, bridge_core::value::Params),
    Option<bridge_core::value::Response>,
);

pub type Handler = fn(
    bridge_core::value::TriggerEvent,
    bridge_core::value::Params,
) -> bridge_core::Result<bridge_core::value::Response>;

pub fn create_middleware_from_handler<'a>(
    handler: Arc<Box<Handler>>,
) -> wasm_lambda_core::router::middleware::Middleware<'a, MiddlewareContext> {
    let handler = handler.clone();
    wasm_lambda_core::router::middleware::Middleware::new(move |context:MiddlewareContext, next:wasm_lambda_core::router::middleware::MiddlewareNext<'a,MiddlewareContext>| {
        let (handler_args, _response) = context;
        let handler_args_clone = handler_args.clone();
        let (event, params) = handler_args;
        let next_response = handler(event,params).unwrap();
        next.call((handler_args_clone,Some(next_response)))
    })
}
