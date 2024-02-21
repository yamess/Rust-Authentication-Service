use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
use actix_web::HttpResponse;
use actix_web::{Error, HttpMessage};
use futures_util::future::LocalBoxFuture;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::future::{ready, Ready};
use std::sync::{Arc, Mutex};

pub fn generate_correlation_id(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}

pub struct TimerMiddleware;

impl<S, B> Transform<S, ServiceRequest> for TimerMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TimerMiddlewareTransform<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TimerMiddlewareTransform { service }))
    }
}

pub struct TimerMiddlewareTransform<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TimerMiddlewareTransform<S>
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
        let start_time = std::time::Instant::now();
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            let elapsed = start_time.elapsed();
            log::info!("Request took: {:?}", elapsed);
            res.headers_mut().insert(
                header::HeaderName::from_static("x-response-time"),
                elapsed.as_millis().to_string().parse().unwrap(),
            );
            Ok(res)
        })
    }
}
