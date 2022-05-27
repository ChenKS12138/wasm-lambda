use hyper::{Body, Response};

use crate::{app::external_control::RequestCtx, json_response};



pub async fn set_module_version(ctx:RequestCtx) -> anyhow::Result<Response<Body>> {
    json_response!(0,"ok","hello")
}

pub async fn create_module_version(ctx:RequestCtx) -> anyhow::Result<Response<Body>> {
    json_response!(0,"ok","hello")
}

pub async fn get_module_version(ctx:RequestCtx) -> anyhow::Result<Response<Body>> {
    json_response!(0,"ok","hello")
}

pub async fn delete_module_version(ctx:RequestCtx) -> anyhow::Result<Response<Body>> {
    json_response!(0,"ok","hello")
}