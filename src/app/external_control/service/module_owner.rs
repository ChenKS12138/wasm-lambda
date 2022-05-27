use hyper::{Body, Response};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{app::external_control::RequestCtx, dao, db_pool, dto, json_response};

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequestDto {
    owner_name: String,
}

pub async fn create_module_owner(ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    let create_user_request_dto = dto!(ctx, CreateUserRequestDto);
    sqlx::query!(
        "INSERT INTO module_owner (owner_name) VALUES (?)",
        create_user_request_dto.owner_name
    )
    .execute(&db_pool!(ctx))
    .await?;
    json_response!(0, "ok", true)
}
