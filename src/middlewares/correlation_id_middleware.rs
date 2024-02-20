use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct CorrelationIdMiddlewareFactory;

impl<S, B> Transform<S, ServiceRequest> for CorrelationIdMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CorrelationIdMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CorrelationIdMiddleware { service }))
    }
}

pub struct CorrelationIdMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CorrelationIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        log::info!("Before request :CorrelationIdMiddleware");

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            log::info!("After Request : CorrelationIdMiddleware");
            Ok(res)
        })
    }
}
