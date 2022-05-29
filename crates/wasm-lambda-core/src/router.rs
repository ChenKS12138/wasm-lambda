use std::{collections::HashMap, hash::Hash, sync::Arc};

use matchit::Router as InternalRouter;

pub struct Route<TMethod, THandler>(TMethod, String, THandler);

impl<TMethod, THandler> Route<TMethod, THandler> {
    pub fn new(method: TMethod, path: &str, handler: THandler) -> Self {
        Self(method, path.to_string(), handler)
    }
}

impl<TMethod: Eq + Hash, THandler> TryInto<Router<TMethod, THandler>> for Route<TMethod, THandler> {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<Router<TMethod, THandler>, Self::Error> {
        let mut router = Router::new();
        router.insert_route(self)?;
        Ok(router)
    }
}

#[derive(Default)]
pub struct Router<TMethod, THandler> {
    pub router_map: HashMap<TMethod, InternalRouter<Arc<THandler>>>,
}

impl<TMethod: Eq + Hash, THandler> Router<TMethod, THandler> {
    pub fn new() -> Self {
        Self {
            router_map: HashMap::new(),
        }
    }
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
    pub fn insert(&mut self, route: Route<TMethod, THandler>) -> anyhow::Result<()> {
        self.router_map
            .entry(route.0)
            .or_insert(InternalRouter::new())
            .insert(route.1, Arc::new(route.2))?;
        Ok(())
    }
    pub fn insert_route(&mut self, route: Route<TMethod, THandler>) -> anyhow::Result<()> {
        self.router_map
            .entry(route.0)
            .or_insert(InternalRouter::new())
            .insert(route.1, Arc::new(route.2))?;
        Ok(())
    }
}
