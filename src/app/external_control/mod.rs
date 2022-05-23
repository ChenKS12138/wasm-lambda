use std::{net::SocketAddr, collections::HashMap, future::Future, sync::Arc};

use async_trait::async_trait;


use hyper::{service::{make_service_fn, service_fn}, Response, Body, Request, Server, Method, StatusCode};
use route_recognizer::{Router as InternalRouter, Params};



#[async_trait]
pub trait Handler:Send+Sync+'static {
    async fn invoke(&self, req: Request<Body>, params: Params) -> anyhow::Result<Response<Body>>;
}


#[async_trait]
impl<F:Send+Sync+'static,FOut> Handler for F where F:Fn(Request<Body>,Params)->FOut,FOut:Future<Output = anyhow::Result<Response<Body>>> + Send +'static{
    async fn invoke(&self, req: Request<Body>, params: Params) -> anyhow::Result<Response<Body>> {
        (self)(req,params).await
    }
}

#[derive(Default)]
struct Router {
    router_map:HashMap< Method,InternalRouter<Box<dyn Handler>>>
}

impl Router {
    async fn handle(router:Arc<Router>,req:Request<Body>) -> anyhow::Result<Response<Body>> {
        if let Some(m) = router.router_map.get(req.method()) {
            if let Ok(m) = m.recognize(req.uri().path()) {
                let handler = m.handler().clone();
                let params = m.params().clone();
                return handler.invoke(req,params).await;
            }
        }
        return not_found(req,Params::default()).await;
    }
}

macro_rules! router_method {
    ($router:ident,$method:path,$path:expr,$handler:expr) => {
        $router.router_map.entry($method).or_insert(InternalRouter::new()).add($path,Box::new($handler));
    };
}



pub async fn make_serve() -> anyhow::Result<()>{
    let mut router:Router = Router::default();

    router_method!(router,Method::GET,"/",index);
    
    let router = Arc::new(router);
    let service = make_service_fn(move |_| { 
        let router = router.clone();
        
        async {
            Ok::<_, anyhow::Error>(service_fn(move |req|{
                Router::handle(router.clone(),req)
            })) 
        }
    });
    let addr: SocketAddr = "0.0.0.0:4444".parse()?;
    Server::bind(&addr).serve(service).await?;
    Ok(())
}

async fn index(_req:Request<Body>,_params:Params) -> anyhow::Result<Response<Body>>{
    Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("Welcome !")).unwrap())
}

async fn not_found(_req:Request<Body>,_params:Params) -> anyhow::Result<Response<Body>>{
    Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("Not Found")).unwrap())
}
