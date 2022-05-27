use std::{collections::HashMap, str::FromStr};

use hyper::{
    header::{HeaderName, HeaderValue},
    Body, HeaderMap, Response, StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use wasmtime::Module;

use crate::{
    app::infra::RequestCtx, body, core, db_pool, http_headers, http_method, json_response,
    path_params,
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct EntryModuleQuery {
    module_env: Option<String>,
    version_digest_value: Option<String>,
    version_raw_value: Option<Vec<u8>>,
}

pub async fn entry(ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    let params = path_params!(ctx);
    let app_name = params.find("app_name").unwrap().clone();
    let version_alias = params.find("version_alias").unwrap().clone();
    let path = "/".to_string() + params.find("path").unwrap_or("").clone();
    let headers = http_headers!(ctx);
    let method = http_method!(ctx);
    let body = body!(ctx).clone();

    let record = sqlx::query_as!(
        EntryModuleQuery,
        r#"SELECT
    module.module_env,
    module_version.version_digest_value,
    module_version.version_raw_value
FROM
    module LEFT JOIN module_version ON module.module_id = module_version.module_id
    WHERE module.module_name = ?
    ORDER BY module_version.create_at DESC
    LIMIT 1
"#,
        app_name
    )
    .fetch_one(&db_pool!(ctx))
    .await?;

    let binary = record.version_raw_value.unwrap();

    let envs = record.module_env.unwrap();
    let envs: HashMap<String, String> = serde_json::from_str(envs.as_str()).unwrap();
    let envs: Vec<(String, String)> = envs.into_iter().map(|(k, v)| (k, v)).collect();

    let mut instance = core::vm::Instance::new(
        move |engine| Ok(Module::from_binary(&engine, &binary)?),
        &envs,
    )?;

    let event_request = bridge::value::TriggerEvent::EventHttpRequest(bridge::value::Request {
        path,
        method,
        headers: headers,
        body: Some(body),
    });

    let event_response = instance.run(event_request).await?;

    Ok(match event_response {
        None => Response::builder()
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .header(
                "X-Module-Digest",
                record.version_digest_value.unwrap_or_default(),
            )
            .body(Body::empty()),
        Some(event_response) => {
            let mut response = Response::builder()
                .status(StatusCode::from_u16(event_response.status as u16)?)
                .header(
                    "X-Module-Digest",
                    record.version_digest_value.unwrap_or_default(),
                );
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
