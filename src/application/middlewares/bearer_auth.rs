use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    sync::Arc,
};
use std::rc::Rc;
use crate::infrastructure::services::validate_bearer_auth_service::VaildateBearerAuthMiddlewareService;
use actix_web::HttpMessage;
pub struct ValidateBearerAuth {
    middleware_service: Arc<dyn VaildateBearerAuthMiddlewareService>,
}
impl ValidateBearerAuth {
    pub fn new(middleware_service: Arc<dyn VaildateBearerAuthMiddlewareService>) -> Self {
        Self { middleware_service }
    }
}
impl<S, B> Transform<S, ServiceRequest> for ValidateBearerAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>+ 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ValidateBearerAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ValidateBearerAuthMiddleware {
            service: Rc::new(service),
            middleware_service: Arc::clone(&self.middleware_service),
        }))
    }
}

pub struct ValidateBearerAuthMiddleware<S> {
    service:  Rc<S>,
    middleware_service: Arc<dyn VaildateBearerAuthMiddlewareService>,
}

impl<S, B> Service<ServiceRequest> for ValidateBearerAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>+ 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let middleware_service = self.middleware_service.clone();
        let bearer_token = req.headers().get("Authorization").cloned();
        let srv = self.service.clone();
        Box::pin(async move {
            let Some(header_value) = bearer_token else {
                return Err(actix_web::error::ErrorBadRequest("not found token"));
            };
            let Ok(token_str) = header_value.to_str() else {
                return Err(actix_web::error::ErrorBadRequest("invalid token format"));
            };
    
            let Some(token) = token_str.strip_prefix("Bearer ") else {
                return Err(actix_web::error::ErrorUnauthorized("missing Bearer prefix"));
            };

            let apporg=middleware_service.authentication(token.to_string()).await?;
            req.extensions_mut().insert(apporg);
            let res = srv.call(req).await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}
