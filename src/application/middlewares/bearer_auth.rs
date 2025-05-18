use super::super::services::validate_bearer_auth_service::ValidateBearerAuthMiddlewareService;
use actix_web::HttpMessage;
use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;
use std::rc::Rc;
use std::{
    future::{Ready, ready},
    sync::Arc,
};
pub struct ValidateBearerAuth {
    middleware_service: Arc<dyn ValidateBearerAuthMiddlewareService>,
}
impl ValidateBearerAuth {
    pub fn new(middleware_service: Arc<dyn ValidateBearerAuthMiddlewareService>) -> Self {
        Self { middleware_service }
    }
}
impl<S, B> Transform<S, ServiceRequest> for ValidateBearerAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
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
    service: Rc<S>,
    middleware_service: Arc<dyn ValidateBearerAuthMiddlewareService>,
}

impl<S, B> Service<ServiceRequest> for ValidateBearerAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let middleware_service = self.middleware_service.clone();
        let srv = self.service.clone();
        Box::pin(async move {
            let token = match req.headers().get("Authorization") {
                Some(hv) => hv.to_str().ok().map(|s| s.to_string()),
                None => None,
            };

            let apporg = middleware_service.authentication(token).await?;
            req.extensions_mut().insert(apporg);
            let res = srv.call(req).await?;

            Ok(res)
        })
    }
}
