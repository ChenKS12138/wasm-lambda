use std::sync::Arc;

use hyper::{Body, Method, Response, StatusCode};

use crate::{
    app::{
        external_control::service,
        infra::{RequestCtx, Router},
    },
    make_route,
};

async fn index(_ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Welcome !"))
        .unwrap())
}

pub fn make_router() -> Arc<Router> {
    let mut router: Router = Router::new();

    // index
    make_route!(router, Method::GET, "/", index);

    // module_owner
    make_route!(
        router,
        Method::POST,
        "/module-owner",
        service::module_owner::create_module_owner
    );

    // module
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
    make_route!(
        router,
        Method::PUT,
        "/module/:module-id/version",
        service::module::set_module_version
    );

    // module_version
    make_route!(
        router,
        Method::GET,
        "/module/:module-id/version",
        service::module_version::get_module_version
    );
    make_route!(
        router,
        Method::POST,
        "/module/:module-id/version",
        service::module_version::create_module_version
    );
    make_route!(
        router,
        Method::DELETE,
        "/module/:module-id/version/:version-id",
        service::module_version::delete_module_version
    );

    Arc::new(router)
}
