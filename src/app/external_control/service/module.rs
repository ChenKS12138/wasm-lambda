use hyper::{Body, Response};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{app::infra::RequestCtx, dao, db_pool, dto, json_response, path_params};

#[derive(Debug, Serialize, Deserialize)]
struct CreateModuleRequestDto {
    #[serde(rename = "moduleName")]
    module_name: String,
    #[serde(rename = "moduleEnv")]
    module_env: serde_json::Value,
}

pub async fn create_module(ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    // TODO owner id
    let owner_id: u32 = 1;
    let create_module_request_dto = dto!(ctx, CreateModuleRequestDto);
    sqlx::query!(
        "INSERT INTO module (module_name,module_env,owner_id) VALUES (?,?,?)",
        create_module_request_dto.module_name,
        create_module_request_dto.module_env,
        owner_id,
    )
    .execute(&db_pool!(ctx))
    .await?;
    json_response!(0, "ok", true)
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct GetModuleResultDto {
    #[serde(rename = "moduleId")]
    module_id: u32,
    #[serde(rename = "moduleName")]
    module_name: String,
    #[serde(rename = "moduleEnv")]
    module_env: Option<String>,
    #[serde(rename = "ownerName")]
    owner_name: Option<String>,
    #[serde(rename = "versionId")]
    version_id: Option<u32>,
}

pub async fn get_module(ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    let records = sqlx::query_as!(
        GetModuleResultDto,
        r#"SELECT 
module.module_id,
module.module_name,
module.module_env,
module_owner.owner_name,
module.version_id
FROM module LEFT JOIN module_owner ON module.owner_id = module_owner.owner_id"#
    )
    .fetch_all(&dao!(ctx).unwrap().pool)
    .await?;
    json_response!(0, "ok", records)
}

#[derive(Debug, Serialize, Deserialize)]
struct SetModuleEnvRequestDto {
    #[serde(rename = "moduleEnv")]
    module_env: serde_json::Value,
}

pub async fn set_module_env(ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    let params = path_params!(ctx);
    let module_id: u32 = params.get("module-id").unwrap().parse()?;
    let set_module_env_request_dto = dto!(ctx, SetModuleEnvRequestDto);
    sqlx::query!(
        "UPDATE module SET module_env = ? WHERE module_id = ?",
        set_module_env_request_dto.module_env,
        module_id
    )
    .execute(&db_pool!(ctx))
    .await?;
    json_response!(0, "ok", true)
}

pub async fn delete_module(ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    let params = path_params!(ctx);
    let module_id: u32 = params.get("module-id").unwrap().parse()?;
    sqlx::query!("DELETE FROM module WHERE module_id = ?", module_id)
        .execute(&db_pool!(ctx))
        .await?;
    json_response!(0, "ok", true)
}

#[derive(Debug, Serialize, Deserialize)]
struct SetModuleVersionRequestDto {
    #[serde(rename = "versionId")]
    version_id: u32,
}

pub async fn set_module_version(ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    let set_module_version_request_dto = dto!(ctx, SetModuleVersionRequestDto);
    let params = path_params!(ctx);
    let module_id: u32 = params.get("module-id").unwrap().parse()?;
    sqlx::query!(
        "UPDATE module SET version_id = ? WHERE module_id = ?",
        set_module_version_request_dto.version_id,
        module_id
    )
    .execute(&db_pool!(ctx))
    .await?;
    json_response!(0, "ok", true)
}
