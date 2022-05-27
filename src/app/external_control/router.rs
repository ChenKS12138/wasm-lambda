use std::sync::Arc;

use hyper::{Body, Method, Response, StatusCode};

use crate::{
    app::external_control::{infra::Router, service},
    make_route,
};

use super::infra::RequestCtx;

async fn index(_ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Welcome !"))
        .unwrap())
}

pub fn make_router() -> Arc<Router> {
    let mut router: Router = Router::default();

    make_route!(router, Method::GET, "/", index);
    make_route!(
        router,
        Method::POST,
        "/module-owner",
        service::module_owner::create_module_owner
    );
    make_route!(
        router,
        Method::POST,
        "/module",
        service::module::create_module
    );
    make_route!(router, Method::GET, "/module", service::module::get_module);
    make_route!(
        router,
        Method::DELETE,
        "/module/:module-id",
        service::module::delete_module
    );
    make_route!(
        router,
        Method::PUT,
        "/module/:module-id/env",
        service::module::set_module_env
    );

    Arc::new(router)
}
