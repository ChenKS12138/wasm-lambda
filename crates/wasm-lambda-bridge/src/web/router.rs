use super::core::middleware;
use bridge_core::value;

use matchit::Router as InternalRouter;

use std::{collections::HashMap, sync::Arc};

pub type Middleware<'a> = middleware::Middleware<'a, MiddlewareContext>;
pub type MiddlewareNext<'a> = middleware::MiddlewareNext<'a, MiddlewareContext>;

pub type MiddlewareContext = (
    (value::TriggerEvent, value::Params),
    Option<value::Response>,
);

#[derive(Debug, Clone, Default)]
pub struct Router<'a> {
    pub prefix: String,
    pub routes: HashMap<(String, String), Arc<Middleware<'a>>>,
}

impl<'a> Router<'a> {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            routes: HashMap::new(),
        }
    }
    pub fn merge(&mut self, other: Self) {
        for (k, v) in other.into_routes() {
            self.routes.insert(k, v);
        }
    }
    pub fn insert(&mut self, method: &str, path: &str, middleware: Arc<Middleware<'a>>) {
        self.routes
            .insert((method.to_string(), path.to_string()), middleware.clone());
    }
    pub fn into_routes(self) -> Vec<((String, String), Arc<Middleware<'a>>)> {
        let prefix = self.prefix.trim_end_matches("/");
        self.routes
            .into_iter()
            .map(|(k, v)| {
                let path = k.1.trim_start_matches("/");
                let path = format!("{}/{}", prefix, path);
                ((k.0, path), v)
            })
            .collect()
    }
    pub fn into_matcher(self) -> HashMap<String, InternalRouter<MiddlewareNext<'a>>> {
        let mut result = HashMap::new();
        for (k, v) in self.into_routes() {
            let router = result.entry(k.0).or_insert(InternalRouter::new());
            router
                .insert(
                    k.1,
                    middleware::Middleware::make_dispatcher(Arc::new(vec![v])),
                )
                .unwrap();
        }
        result
    }
}

impl<'a> Into<Middleware<'a>> for Router<'a> {
    fn into(self) -> Middleware<'a> {
        let matcher = Arc::new(self.into_matcher());
        middleware::Middleware::new(Arc::new(Box::new(move |context, next| {
            if context.1.is_some() {
                return context;
            }
            let context_clone = context.clone();
            let request = match context_clone.0 .0 {
                value::TriggerEvent::EventHttpRequest(request) => request,
                value::TriggerEvent::EventInternalModuleCall(_, request) => request,
            };
            let split_idx: usize = request.path.find("?").unwrap_or(request.path.len());

            let dispatcher = matcher
                .get(&request.method)
                .and_then(|v| v.at(&request.path[..split_idx]).ok());
            if let Some(dispatcher) = dispatcher {
                let params: HashMap<String, String> = dispatcher
                    .params
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect();
                let dispatcher = dispatcher.value;
                return next.call(dispatcher.call(((context.0 .0, params), context.1)));
            }
            next.call(context)
        })))
    }
}

impl<'a> Into<Middleware<'a>> for Box<dyn Fn() -> Router<'a>> {
    fn into(self) -> Middleware<'a> {
        self().into()
    }
}

pub struct Handler(
    Arc<Box<dyn Fn(value::TriggerEvent, value::Params) -> anyhow::Result<value::Response>>>,
);

impl Handler {
    pub fn new(
        f: impl Fn(value::TriggerEvent, value::Params) -> anyhow::Result<value::Response> + 'static,
    ) -> Self {
        Self(Arc::new(Box::new(f)))
    }
}

impl<'a> Into<Middleware<'a>> for Handler {
    fn into(self) -> Middleware<'a> {
        let self_clone = self.0.clone();
        middleware::Middleware::new(Arc::new(Box::new(move |context, next| {
            if context.1.is_some() {
                return context;
            }
            let request_clone = context.0.clone();
            let request = context.0;
            let response = self_clone(request.0, request.1).unwrap();
            next.call((request_clone, Some(response)))
        })))
    }
}
