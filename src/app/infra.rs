use std::{collections::HashMap, future::Future, sync::Arc};

use async_trait::async_trait;
use hyper::{Body, Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use wasm_lambda_core::router;

use crate::{core::vm::Environment, db::dao::Dao};

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

pub struct Router(pub router::Router<hyper::Method, Box<dyn Handler>>);

impl Router {
    pub fn new() -> Self {
        Self(router::Router::new())
    }
    pub async fn handle(
        router: Arc<Router>,
        req: Request<Body>,
        app_state: AppState,
    ) -> anyhow::Result<Response<Body>> {
        if let Some((handler, params)) = router.0.search(&req.method(), req.uri().path()) {
            return handler
                .invoke(RequestCtx {
                    request: req,
                    params,
                    app_state: app_state.clone(),
                })
                .await;
        } else {
            return not_found(RequestCtx {
                request: req,
                params: HashMap::default(),
                app_state: app_state.clone(),
            })
            .await;
        }
    }
}

#[macro_export]
macro_rules! make_route {
    ($router:ident,$method:path,$path:expr,$handler:expr) => {
        $router.0.insert(wasm_lambda_core::router::Route::new($method, $path, Box::new($handler))).unwrap();
    };
    ($route:ident,[ $($method:path),*],$path:expr,$handler:expr ) => {
        $(
            make_route!($route, $method, $path, $handler);
        )*
    };
}

async fn not_found(_ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .unwrap())
}

pub struct RequestCtx {
    pub request: Request<Body>,
    pub params: HashMap<String, String>,
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

#[macro_export]
macro_rules! http_method {
    ($ctx:expr) => {
        $ctx.request.method().to_string()
    };
}

#[macro_export]
macro_rules! http_headers {
    ($ctx:expr) => {
        $ctx.request
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
            .collect::<std::collections::HashMap<String, String>>()
    };
}

#[derive(Clone)]
pub struct AppState {
    pub dao: Arc<Dao>,
    pub environment: Arc<Environment>,
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
        crate::app::infra::JsonResult::new(
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
