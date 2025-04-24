use std::future::{ready, Ready};
use actix_web::{ dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, };
use futures_util::future::LocalBoxFuture;

pub struct ValidateBearerAuth;
impl<S, B> Transform<S, ServiceRequest> for ValidateBearerAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ValidateBearerAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ValidateBearerAuthMiddleware { service }))
    }
}

pub struct ValidateBearerAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ValidateBearerAuthMiddleware<S>
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
        println!("Hi from start. You requested: {:?}", req.headers());
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}