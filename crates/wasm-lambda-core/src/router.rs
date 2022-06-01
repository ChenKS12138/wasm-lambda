use std::{collections::HashMap, hash::Hash, sync::Arc};

use matchit::Router as InternalRouter;

#[derive(Default, Clone, Debug)]
pub struct Route<TMethod, THandler>(TMethod, String, THandler);

impl<TMethod, THandler> Route<TMethod, THandler> {
    pub fn new(method: TMethod, path: &str, handler: THandler) -> Self {
        Self(method, path.to_string(), handler)
    }
}

impl<TMethod: Eq + Hash, THandler> Into<RouteMap<TMethod, THandler>> for Route<TMethod, THandler> {
    fn into(self) -> RouteMap<TMethod, THandler> {
        let mut route_map = RouteMap::new();
        route_map.insert_route(self).unwrap();
        route_map
    }
}

#[derive(Default, Clone, Debug)]
pub struct RouteMap<TMethod, THandler> {
    router_map: HashMap<(TMethod, String), THandler>,
}

impl<TMethod: Eq + Hash, THandler> RouteMap<TMethod, THandler> {
    pub fn new() -> Self {
        Self {
            router_map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, other: impl Into<RouteMap<TMethod, THandler>>) -> anyhow::Result<()> {
        self.router_map.extend(other.into().router_map);
        Ok(())
    }
    pub fn insert_route(&mut self, route: Route<TMethod, THandler>) -> anyhow::Result<()> {
        self.router_map.insert((route.0, route.1), route.2);
        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct Router<TMethod, THandler> {
    pub router_map: HashMap<TMethod, InternalRouter<Arc<THandler>>>,
}

impl<TMethod: Eq + Hash, THandler> Router<TMethod, THandler> {
    pub fn search(
        &self,
        method: &TMethod,
        path: &str,
    ) -> Option<(Arc<THandler>, HashMap<String, String>)> {
        self.router_map
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

impl<TMethod: Eq + Hash, THandler> From<RouteMap<TMethod, THandler>> for Router<TMethod, THandler> {
    fn from(route_map: RouteMap<TMethod, THandler>) -> Self {
        let mut router: Router<TMethod, THandler> = Router {
            router_map: HashMap::new(),
        };
        for (key, value) in route_map.router_map {
            router
                .router_map
                .entry(key.0)
                .or_insert(InternalRouter::new())
                .insert(key.1, Arc::new(value))
                .unwrap();
        }
        router
    }
}
