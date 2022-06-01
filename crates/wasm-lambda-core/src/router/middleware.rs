use std::{ops::Deref, sync::Arc};

pub enum MiddlewareNext<'a, TContext> {
    Next(Box<dyn 'a + Fn(TContext) -> TContext>),
    None,
}

impl<'a, TContext> MiddlewareNext<'a, TContext> {
    pub fn call(self, value: TContext) -> TContext {
        match self {
            MiddlewareNext::Next(next) => next(value),
            MiddlewareNext::None => value,
        }
    }
}

pub struct Middleware<'a, TContext>(
    Box<dyn 'a + Fn(TContext, MiddlewareNext<'a, TContext>) -> TContext + Sync + Send>,
);

impl<'a, TContext> Middleware<'a, TContext> {
    pub fn new(
        middleware: impl 'a + Fn(TContext, MiddlewareNext<'a, TContext>) -> TContext + 'a + Sync + Send,
    ) -> Self {
        Self(Box::new(middleware))
    }
}

impl<'a, TContext> Deref for Middleware<'a, TContext> {
    type Target =
        Box<(dyn Fn(TContext, MiddlewareNext<'a, TContext>) -> TContext + Send + Sync + 'a)>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, TContext> std::fmt::Debug for Middleware<'a, TContext> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Middleware]")
    }
}

pub type Middlewares<'a, TContext> = Arc<Vec<Arc<Middleware<'a, TContext>>>>;

fn make_next<'a, TContext: 'a>(
    middlewares: Middlewares<'a, TContext>,
    index: usize,
) -> MiddlewareNext<'a, TContext> {
    let middlewares_clone = middlewares.clone();
    match middlewares.get(index) {
        None => MiddlewareNext::None,
        Some(middleware) => {
            let middleware = middleware.clone();
            MiddlewareNext::Next(Box::new(move |value: TContext| -> TContext {
                (*middleware)(value, make_next(middlewares_clone.clone(), index + 1))
            }))
        }
    }
}

pub fn compose<'a, TContext: 'a>(
    init_value: TContext,
    middlewares: Middlewares<'a, TContext>,
) -> TContext {
    let next = make_next(middlewares.clone(), 0);
    next.call(init_value)
}
