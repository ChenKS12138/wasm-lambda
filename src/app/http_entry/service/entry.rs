use std::str::FromStr;

use hyper::{
    header::{HeaderName, HeaderValue},
    Body, Response, StatusCode,
};

use crate::{app::infra::RequestCtx, body, http_headers, http_method, path_params};

pub async fn entry(ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    let params = path_params!(ctx);
    let module_name = params.get("module_name").unwrap().clone();
    let version_alias = params.get("version_alias").unwrap().clone();
    let path = params
        .get("path")
        .and_then(|v| Some(v.clone()))
        .unwrap_or("/".to_string());
    let headers = http_headers!(ctx);
    let method = http_method!(ctx);
    let query = ctx
        .request
        .uri()
        .query()
        .and_then(|v| Some(format!("?{}", v)))
        .unwrap_or_default();
    let body = body!(ctx).clone();

    let event_request = bridge::value::TriggerEvent::EventHttpRequest(bridge::value::Request {
        path: path + &query,
        method,
        headers: headers,
        body: Some(body),
    });

    let (event_response, version_digest) = ctx
        .app_state
        .environment
        .call(module_name, version_alias, event_request)
        .await?;

    Ok(match event_response {
        None => Response::builder()
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .header("X-Module-Digest", version_digest)
            .body(Body::empty()),
        Some(event_response) => {
            let mut response = Response::builder()
                .status(StatusCode::from_u16(event_response.status as u16)?)
                .header("X-Module-Digest", version_digest);
            if let Some(headers) = response.headers_mut() {
                for (k, v) in event_response.headers {
                    headers.append(HeaderName::from_str(&k)?, HeaderValue::from_str(&v)?);
                }
            }
            response.body(match event_response.body {
                None => Body::empty(),
                Some(body) => Body::from(body),
            })
        }
    }
    .unwrap())
}
