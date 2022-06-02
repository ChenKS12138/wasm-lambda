// TODO remove this
#![allow(dead_code)]

use std::{collections::HashMap, hash::Hash, sync::Arc};

use super::middleware;
use matchit::Router as InternalRouter;

#[derive(Default, Clone, Debug)]
pub struct Route<TMethod, THandler>(TMethod, String, Arc<THandler>);

impl<TMethod, THandler> Route<TMethod, THandler> {
    pub fn new(method: TMethod, path: &str, handler: THandler) -> Self {
        Self(method, path.to_string(), Arc::new(handler))
    }
}

impl<'a, TMethod: Eq + Hash + Clone, THandler: Clone, TMiddlewareContext>
    Into<RouteMap<'a, TMethod, THandler, TMiddlewareContext>> for Route<TMethod, THandler>
{
    fn into(self) -> RouteMap<'a, TMethod, THandler, TMiddlewareContext> {
        let mut route_map: RouteMap<'a, _, _, _> = RouteMap::new();
        route_map.insert_route(self).unwrap();
        route_map
    }
}

#[derive(Default)]
pub struct RouteMap<'a, TMethod: Clone, THandler: Clone, TMiddlewareContext> {
    prefix: String,
    routes: HashMap<
        (TMethod, String),
        (
            Arc<THandler>,
            middleware::Middlewares<'a, TMiddlewareContext>,
        ),
    >,
}

impl<'a, TMethod: Eq + Hash + Clone, THandler: Clone, TMiddlewareContext>
    RouteMap<'a, TMethod, THandler, TMiddlewareContext>
{
    pub fn new() -> Self {
        Self {
            prefix: String::new(),
            routes: HashMap::new(),
        }
    }
    pub fn insert(
        &mut self,
        other: RouteMap<'a, TMethod, THandler, TMiddlewareContext>,
    ) -> anyhow::Result<()> {
        let other: RouteMap<'a, TMethod, THandler, TMiddlewareContext> = other.into();
        self.routes.extend(other.routes);
        Ok(())
    }
    pub fn insert_route(&mut self, route: Route<TMethod, THandler>) -> anyhow::Result<()> {
        self.routes
            .insert((route.0, route.1), (route.2, Arc::new(vec![])));
        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct Router<'a, TMethod, THandler: Clone, TMiddlewareContext> {
    pub routes: HashMap<
        TMethod,
        InternalRouter<(
            Arc<THandler>,
            middleware::Middlewares<'a, TMiddlewareContext>,
        )>,
    >,
}

impl<'a, TMethod: Eq + Hash + Clone, THandler: Clone, TMiddlewareContext>
    Router<'a, TMethod, THandler, TMiddlewareContext>
{
    pub fn search(
        &self,
        method: &TMethod,
        path: &str,
    ) -> Option<(
        (
            Arc<THandler>,
            middleware::Middlewares<'a, TMiddlewareContext>,
        ),
        HashMap<String, String>,
    )> {
        self.routes
            .get(method)
            .and_then(|v| v.at(path).ok())
            .and_then(|v| {
                Some((
                    v.value.clone(),
                    v.params
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<HashMap<String, String>>(),
                ))
            })
    }
}

impl<'a, TMethod: Eq + Hash + Clone, THandler: Clone, TMiddlewareContext>
    From<RouteMap<'a, TMethod, THandler, TMiddlewareContext>>
    for Router<'a, TMethod, THandler, TMiddlewareContext>
{
    fn from(route_map: RouteMap<'a, TMethod, THandler, TMiddlewareContext>) -> Self {
        let mut router: Router<'a, TMethod, THandler, TMiddlewareContext> = Router {
            routes: HashMap::new(),
        };
        for (key, value) in route_map.routes {
            router
                .routes
                .entry(key.0)
                .or_insert(InternalRouter::new())
                .insert(key.1, value.clone())
                .unwrap();
        }
        router
    }
}
