mod core;
mod extractor;
mod response;
mod router;

pub use extractor::*;
pub use response::*;
pub use router::*;

#[macro_export]
macro_rules! compose_routers {
    ($prefix:expr,[$($make_routers:expr),*]) => {
        || -> wasm_lambda_bridge::web::Router<'_> {
            let mut router = wasm_lambda_bridge::web::Router::new($prefix);
            $(
                router.merge($make_routers());
            )*
            router
        }
    };
}

#[macro_export]
macro_rules! dispatch {
    ($event:expr,[$($make_middlewares:expr),*]) => {
        {
            let dispatcher = wasm_lambda_bridge::web::Middleware::make_dispatcher(std::sync::Arc::new(vec![
                    $(
                        std::sync::Arc::new({$make_middlewares()}.into()),
                    )*
            ]));
            let context = dispatcher.call((($event, std::collections::HashMap::new()),None));
            Ok(context.1.unwrap())
        }
    };
}
