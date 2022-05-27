use hyper::{Body, Response};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{
    app::external_control::RequestCtx, dao, db_pool, dto, json_response, path_params, query_string,
};

#[derive(Debug, Serialize, Deserialize)]
struct CreateModuleRequestDto {
    module_name: String,
    module_env: serde_json::Value,
}

pub async fn create_module(ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    let create_module_request_dto = dto!(ctx, CreateModuleRequestDto);
    sqlx::query!(
        "INSERT INTO module (module_name,module_env,owner_id) VALUES (?,?,1)",
        create_module_request_dto.module_name,
        create_module_request_dto.module_env
    )
    .execute(&db_pool!(ctx))
    .await?;
    json_response!(0, "ok", true)
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct GetModuleResultDto {
    module_id: u32,
    module_name: String,
    module_env: Option<String>,
    owner_name: Option<String>,
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
    .fetch_all(&dao!(ctx).pool)
    .await?;
    json_response!(0, "ok", records)
}

#[derive(Debug, Serialize, Deserialize)]
struct SetModuleEnvRequestDto {
    module_env: serde_json::Value,
}

pub async fn set_module_env(ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    let params = path_params!(ctx);
    let module_id: u32 = params.find("module-id").unwrap().parse()?;
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
    let module_id: u32 = params.find("module-id").unwrap().parse()?;
    sqlx::query!("DELETE FROM module WHERE module_id = ?", module_id)
        .execute(&db_pool!(ctx))
        .await?;
    json_response!(0, "ok", true)
}
