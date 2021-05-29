use actix_web::{
    dev::ServiceRequest,
    dev::{Service, ServiceResponse, Transform},
    http::header::{HeaderValue, CACHE_CONTROL},
    Error,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// Custom middle ware to set the response `CACHE_CONTROL` header to the passed Duration
pub struct CacheInterceptor(pub Duration);

impl<S, B> Transform<S> for CacheInterceptor
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CacheInterceptorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CacheInterceptorMiddleware {
            service,
            duration: self.0,
        })
    }
}

pub struct CacheInterceptorMiddleware<S> {
    service: S,
    duration: Duration,
}

impl<S, B> Service for CacheInterceptorMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        let duration = self.duration.clone();
        Box::pin(async move {
            let mut res = fut.await?;
            let headers = res.headers_mut();

            headers.append(
                CACHE_CONTROL,
                HeaderValue::from_str(&format!("max-age={}", duration.as_secs())).unwrap(),
            );
            return Ok(res);
        })
    }
}
