pub mod infra;
pub mod router;
pub mod service;

use std::{net::SocketAddr, sync::Arc};

use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};

use crate::{app::external_control::infra::AppState, db::dao::Dao};

use self::{
    infra::{RequestCtx, Router},
    router::make_router,
};

pub async fn make_serve(dao: Arc<Dao>) -> anyhow::Result<()> {
    let app_state = AppState { dao };
    let router = make_router();

    let service = make_service_fn(move |_| {
        let router = router.clone();
        let app_state = app_state.clone();

        async {
            Ok::<_, anyhow::Error>(service_fn(move |req| {
                Router::handle(router.clone(), req, app_state.clone())
            }))
        }
    });
    let addr: SocketAddr = "0.0.0.0:4444".parse()?;
    Server::bind(&addr).serve(service).await?;
    Ok(())
}
