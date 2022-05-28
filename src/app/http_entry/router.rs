use std::sync::Arc;

use hyper::Method;

use crate::{
    app::{http_entry::service, infra::Router},
    make_route,
};

pub fn make_router() -> Arc<Router> {
    let mut router: Router = Router::default();

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
        "/:app_name/:version_alias",
        service::entry::entry
    );

    Arc::new(router)
}
