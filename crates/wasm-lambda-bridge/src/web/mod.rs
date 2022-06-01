mod extractor;
mod middleware;
mod response;

pub use extractor::*;
pub use middleware::*;
pub use response::*;

pub use wasm_lambda_core::router;

pub type RoutesCompose = wasm_lambda_core::router::RouteMap<
    'static,
    String,
    Box<crate::web::Handler>,
    crate::web::MiddlewareContext,
>;

// macro_rules! make_routes_middleware {
//     ($prefix:expr) => {

//     };
// }

// #[macro_export]
// macro_rules! compose_routes {
//     ($prefix:expr,[$($make_route:expr),*]) => {
//         {
//             let mut route_map = wasm_lambda_bridge::web::router::RouteMap::new();
//             $(
//                 let other:wasm_lambda_bridge::web::router::RouteMap<'_,_,_,wasm_lambda_bridge::web::MiddlewareContext> = $make_route().into();
//                 route_map.insert(other).unwrap();
//             )*
//             route_map
//         }
//     };
// }

// #[macro_export]
// macro_rules! handle_event {
//     ($event:expr,$route_map:expr) => {{
//         let router: wasm_lambda_bridge::web::router::Router<
//             '_,
//             _,
//             _,
//             wasm_lambda_bridge::web::MiddlewareContext,
//         > = $route_map.into();

//         let ((handler, middlewares), params, event) = match $event {
//             wasm_lambda_bridge::core::value::TriggerEvent::EventHttpRequest(request) => {
//                 let split_idx: usize = request.path.find("?").unwrap_or(request.path.len());
//                 let (m, params) = router
//                     .search(&request.method, &request.path[..split_idx])
//                     .unwrap();
//                 (
//                     m,
//                     params,
//                     wasm_lambda_bridge::core::value::TriggerEvent::EventHttpRequest(request),
//                 )
//             }
//             wasm_lambda_bridge::core::value::TriggerEvent::EventInternalModuleCall(
//                 module_name,
//                 request,
//             ) => {
//                 let split_idx: usize = request.path.find("?").unwrap_or(request.path.len());
//                 let (m, params) = router
//                     .search(&request.method, &request.path[..split_idx])
//                     .unwrap();
//                 (
//                     m,
//                     params,
//                     wasm_lambda_bridge::core::value::TriggerEvent::EventInternalModuleCall(
//                         module_name,
//                         request,
//                     ),
//                 )
//             }
//         };
//         let end_middleware = wasm_lambda_bridge::web::create_middleware_from_handler(handler);
//         let mut middlewares = (*middlewares).clone();
//         middlewares.push(std::sync::Arc::new(end_middleware));
//         let context = wasm_lambda_bridge::web::router::middleware::compose(
//             ((event, params), None),
//             std::sync::Arc::new(middlewares),
//         );
//         Ok(context.1.unwrap())
//     }};
// }
