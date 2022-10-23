use std::sync::Arc;

use hyper::Method;

use crate::{
    app::{
        http_entry::service,
        infra::{RouteMap, Router},
    },
    make_route,
};

pub fn make_router() -> Arc<Router> {
    let mut router = RouteMap::new();

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
        "/:module_name/*path",
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
        "/:module_name",
        service::entry::entry
    );

    Arc::new(router.into())
}
