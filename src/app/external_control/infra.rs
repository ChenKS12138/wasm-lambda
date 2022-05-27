use std::{collections::HashMap, future::Future, sync::Arc};

use async_trait::async_trait;
use hyper::{Body, Method, Request, Response, StatusCode};
use route_recognizer::{Params, Router as InternalRouter};
use serde::{Deserialize, Serialize};

use crate::db::dao::Dao;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn invoke(&self, ctx: RequestCtx) -> anyhow::Result<Response<Body>>;
}

#[async_trait]
impl<F: Send + Sync + 'static, FOut> Handler for F
where
    F: Fn(RequestCtx) -> FOut,
    FOut: Future<Output = anyhow::Result<Response<Body>>> + Send + 'static,
{
    async fn invoke(&self, ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
        (self)(ctx).await
    }
}

#[derive(Default)]
pub struct Router {
    pub router_map: HashMap<Method, InternalRouter<Box<dyn Handler>>>,
}

#[macro_export]
macro_rules! make_route {
    ($router:ident,$method:path,$path:expr,$handler:expr) => {
        $router
            .router_map
            .entry($method)
            .or_insert(route_recognizer::Router::new())
            .add($path, Box::new($handler));
    };
}

impl Router {
    pub async fn handle(
        router: Arc<Router>,
        req: Request<Body>,
        app_state: AppState,
    ) -> anyhow::Result<Response<Body>> {
        if let Some(m) = router.router_map.get(req.method()) {
            if let Ok(m) = m.recognize(req.uri().path()) {
                let handler = m.handler().clone();
                let params = m.params().clone();
                return handler
                    .invoke(RequestCtx {
                        request: req,
                        params,
                        app_state: app_state.clone(),
                    })
                    .await;
            }
        }
        return not_found(RequestCtx {
            request: req,
            params: Params::default(),
            app_state: app_state.clone(),
        })
        .await;
    }
}

async fn not_found(_ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .unwrap())
}

pub struct RequestCtx {
    pub request: Request<Body>,
    pub params: Params,
    pub app_state: AppState,
}

#[macro_export]
macro_rules! body {
    ($ctx:expr) => {
        &hyper::body::to_bytes($ctx.request.into_body())
            .await?
            .to_vec()
    };
}

#[macro_export]
macro_rules! dto {
    ($ctx:expr,$dto:ty) => {
        serde_json::from_slice::<$dto>(crate::body!($ctx))?
    };
}

#[macro_export]
macro_rules! dao {
    ($ctx:expr) => {
        $ctx.app_state.dao
    };
}

#[macro_export]
macro_rules! db_pool {
    ($ctx:expr) => {
        $ctx.app_state.dao.pool
    };
}

#[macro_export]
macro_rules! query_string {
    ($ctx:expr) => {
        $ctx.request.uri().query().and_then(|q| {
            Some(
                url::form_urlencoded::parse(q.as_ref())
                    .into_owned()
                    .collect::<std::collections::HashMap<String, String>>(),
            )
        })
    };
}

#[macro_export]
macro_rules! path_params {
    ($ctx:expr) => {
        $ctx.params
    };
}

#[derive(Clone)]
pub struct AppState {
    pub dao: Arc<Dao>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonResult {
    pub code: u64,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl JsonResult {
    pub fn new(code: u64, message: String, data: Option<serde_json::Value>) -> Self {
        Self {
            code,
            message,
            data,
        }
    }
}

impl From<JsonResult> for Body {
    fn from(r: JsonResult) -> Self {
        Body::from(serde_json::to_string(&r).unwrap())
    }
}

#[macro_export]
macro_rules! json_result {
    ($code:expr,$message:expr,$data:expr) => {
        crate::app::external_control::infra::JsonResult::new(
            $code,
            String::from($message),
            Some(serde_json::Value::from(serde_json::to_value($data)?)),
        )
    };
}

#[macro_export]
macro_rules! json_response {
    ($code:expr,$message:expr,$data:expr) => {
        hyper::Response::builder()
            .status(hyper::StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(hyper::Body::from(crate::json_result!(
                $code, $message, $data
            )))
            .map_err(|err| anyhow::Error::from(err))
    };
}
