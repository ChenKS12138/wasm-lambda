pub mod router;
pub mod service;

use std::{net::SocketAddr, sync::Arc};

use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};

use crate::{core::vm::Environment, db::dao::Dao};

use super::infra::{AppState, Router};

pub async fn make_serve(app_state: AppState) -> anyhow::Result<()> {
    let router = router::make_router();

    let service = make_service_fn(move |_| {
        let router = router.clone();
        let app_state = app_state.clone();

        async {
            Ok::<_, anyhow::Error>(service_fn(move |req| {
                Router::handle(router.clone(), req, app_state.clone())
            }))
        }
    });

    let addr: SocketAddr = "0.0.0.0:3333".parse()?;
    Server::bind(&addr).serve(service).await?;
    Ok(())
}
