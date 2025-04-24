use std::{future::{ready, Ready}, sync::Arc};
use actix_web::{ dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, };
use futures_util::future::LocalBoxFuture;

use crate::infrastructure::services::validate_bearer_auth_service::VaildateBearerAuthMiddlewareService;

pub struct ValidateBearerAuth{
    middleware_service: Arc<dyn VaildateBearerAuthMiddlewareService>
}
impl ValidateBearerAuth {
    pub fn new (middleware_service:Arc<dyn VaildateBearerAuthMiddlewareService>)->Self{
        Self { middleware_service }
    }
}
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
        ready(Ok(ValidateBearerAuthMiddleware { service ,middleware_service:Arc::clone(&self.middleware_service)}))
    }
}

pub struct ValidateBearerAuthMiddleware<S> {
    service: S,
    middleware_service: Arc<dyn VaildateBearerAuthMiddlewareService>
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

        println!("Hi from start. You requested: {:?}", req.headers().get("Authorization").cloned());
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}