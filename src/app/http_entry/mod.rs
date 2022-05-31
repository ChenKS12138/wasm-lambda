pub mod router;
pub mod service;

use std::net::SocketAddr;

use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};

use super::infra::{router_handle, AppState};

pub async fn make_serve(bind_addr: &str, app_state: AppState) -> anyhow::Result<()> {
    let router = router::make_router();

    let service = make_service_fn(move |_| {
        let router = router.clone();
        let app_state = app_state.clone();

        async {
            Ok::<_, anyhow::Error>(service_fn(move |req| {
                router_handle(router.clone(), req, app_state.clone())
            }))
        }
    });

    let addr: SocketAddr = bind_addr.parse()?;
    Server::bind(&addr).serve(service).await?;
    Ok(())
}
