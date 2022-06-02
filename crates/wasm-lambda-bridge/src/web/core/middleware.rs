use std::{ops::Deref, sync::Arc};

pub enum MiddlewareNext<'a, TContext> {
    Next(Box<dyn 'a + Fn(TContext) -> TContext>),
    None,
}

impl<'a, TContext> MiddlewareNext<'a, TContext> {
    pub fn call(&self, value: TContext) -> TContext {
        match &self {
            MiddlewareNext::Next(next) => next(value),
            MiddlewareNext::None => value,
        }
    }
}

pub struct Middleware<'a, TContext: 'a>(
    Arc<Box<dyn 'a + Fn(TContext, MiddlewareNext<'a, TContext>) -> TContext>>,
);

impl<'a, TContext> Middleware<'a, TContext> {
    pub fn new(
        middleware: Arc<Box<dyn 'a + Fn(TContext, MiddlewareNext<'a, TContext>) -> TContext>>,
    ) -> Self {
        Self(middleware)
    }
    fn make_next(
        middlewares: Middlewares<'a, TContext>,
        index: usize,
    ) -> MiddlewareNext<'a, TContext> {
        let middlewares_clone = middlewares.clone();
        match middlewares.get(index) {
            None => MiddlewareNext::None,
            Some(middleware) => {
                let middleware = middleware.clone();
                MiddlewareNext::Next(Box::new(move |value: TContext| -> TContext {
                    (*middleware)(
                        value,
                        Middleware::make_next(middlewares_clone.clone(), index + 1),
                    )
                }))
            }
        }
    }
    pub fn make_dispatcher(middlewares: Middlewares<'a, TContext>) -> MiddlewareNext<'a, TContext> {
        Middleware::make_next(middlewares.clone(), 0)
    }
}

impl<'a, TContext> Deref for Middleware<'a, TContext> {
    type Target = Arc<Box<(dyn Fn(TContext, MiddlewareNext<'a, TContext>) -> TContext + 'a)>>;
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

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn middlewares_works() {
        let m1: Middleware<'_, i32> = Middleware::new(Arc::new(Box::new(move |context, next| {
            assert_eq!(context, 0);
            let next_context = next.call(context + 1) + 1;
            assert_eq!(next_context, 28);
            next_context + 2
        })));
        let m2: Middleware<'_, i32> = Middleware::new(Arc::new(Box::new(move |context, next| {
            assert_eq!(context, 1);
            let next_context = next.call(context + 10) + 2;
            assert_eq!(next_context, 27);
            next_context
        })));
        let m3: Middleware<'_, i32> = Middleware::new(Arc::new(Box::new(move |context, next| {
            assert_eq!(context, 11);
            let next_context = next.call(context * 2);
            assert_eq!(next_context, 22);
            next_context + 3
        })));
        let middlewares: Middlewares<'_, i32> =
            Arc::new(vec![Arc::new(m1), Arc::new(m2), Arc::new(m3)]);
        let dispatcher = Middleware::make_dispatcher(middlewares);
        assert_eq!(dispatcher.call(0), 30);
    }
}
