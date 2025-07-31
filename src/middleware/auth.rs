use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error,
};
use futures::future::{ok, Ready};
use std::future::{ready, Future};
use std::pin::Pin;

pub struct Auth {
    secret: String,
}

impl Auth {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { 
            service,
            secret: self.secret.clone(),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
    secret: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers()
            .get("accesskey")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.trim());
        
        // Check if the token matches the secret
        if let Some(token_str) = token {
            if token_str == self.secret {
                return Box::pin(self.service.call(req));
            }
        }

        // Return an error if authentication fails
        Box::pin(ready(Err(ErrorUnauthorized("Access forbidden"))))
    }
}