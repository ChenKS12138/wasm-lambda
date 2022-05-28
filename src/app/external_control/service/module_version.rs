use hyper::{Body, Response};
use serde::{Deserialize, Serialize};

use hex;
use ring::digest;

use crate::{app::infra::RequestCtx, body, db_pool, json_response, path_params};

#[derive(Debug, Serialize, Deserialize)]
struct GetModuleVersionResultDto {
    #[serde(rename = "versionId")]
    version_id: u32,
    #[serde(rename = "versionDigestValue")]
    version_digest_value: String,
}

pub async fn get_module_version(ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    let params = path_params!(ctx);
    let module_id: u32 = params.get("module-id").unwrap().parse()?;
    let records = sqlx::query_as!(
        GetModuleVersionResultDto,
        r#"SELECT version_id,version_digest_value FROM module_version WHERE module_id = ?"#,
        module_id
    )
    .fetch_all(&db_pool!(ctx))
    .await?;
    json_response!(0, "ok", records)
}

pub async fn create_module_version(ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    let params = path_params!(ctx);
    let module_id: u32 = params.get("module-id").unwrap().parse()?;
    let body = body!(ctx).clone();
    let digest_result = digest::digest(&digest::SHA256, &body);
    let digest_value = hex::encode(digest_result.as_ref());

    let precompile = ctx.app_state.environment.engine.precompile_module(&body)?;

    sqlx::query!(
        r#"INSERT INTO module_version( module_id, version_digest_value, version_raw_value,version_precompile) VALUES (?,?,?,?)"#,
        module_id,digest_value,body,precompile
    )
    .execute(&db_pool!(ctx))
    .await?;
    json_response!(0, "ok", true)
}

pub async fn delete_module_version(ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    let params = path_params!(ctx);
    let module_id: u32 = params.get("module-id").unwrap().parse()?;
    let version_id: u32 = params.get("version-id").unwrap().parse()?;

    sqlx::query!(
        r#"DELETE FROM module_version WHERE module_id = ? AND version_id = ?"#,
        module_id,
        version_id
    )
    .execute(&db_pool!(ctx))
    .await?;

    json_response!(0, "ok", true)
}
