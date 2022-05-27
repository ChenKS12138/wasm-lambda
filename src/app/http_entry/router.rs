use std::sync::Arc;

use hyper::{Body, Method, Response, StatusCode};

use crate::{
    app::{
        http_entry::service,
        infra::{RequestCtx, Router},
    },
    make_route,
};

// async fn index(_ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
//     Ok(Response::builder()
//         .status(StatusCode::OK)
//         .body(Body::from("Welcome !"))
//         .unwrap())
// }

pub fn make_router() -> Arc<Router> {
    let mut router: Router = Router::default();

    // make_route!(router, Method::GET, "/", index);

    // TODO make route macro
    make_route!(
        router,
        [
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
            Method::OPTIONS,
            Method::HEAD,
            Method::CONNECT,
            Method::TRACE
        ],
        "/:app_name/:version_alias/*path",
        service::entry::entry
    );

    make_route!(
        router,
        [
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
            Method::OPTIONS,
            Method::HEAD,
            Method::CONNECT,
            Method::TRACE
        ],
        "/:app_name/:version_alias/",
        service::entry::entry
    );

    make_route!(
        router,
        [
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
            Method::OPTIONS,
            Method::HEAD,
            Method::CONNECT,
            Method::TRACE
        ],
        "/:app_name/:version_alias",
        service::entry::entry
    );

    Arc::new(router)
}
