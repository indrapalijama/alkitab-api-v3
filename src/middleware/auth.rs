use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error,
};
use futures::future::{ok, Ready};
use std::future::{ready, Future};
use std::pin::Pin;
use std::env;

pub struct Auth;

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
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
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
        
        // Check if SECRET environment variable is set
        let secret = match env::var("SECRET") {
            Ok(secret) => secret,
            Err(_) => {
                println!("ERROR: SECRET environment variable is not set. Please set it in your .env file.");
                return Box::pin(async move {
                    Err(ErrorUnauthorized("Server configuration error: SECRET not set"))
                });
            }
        };

        // Check if the token matches the secret
        if let Some(token_str) = token {
            if token_str == secret {
                let fut = self.service.call(req);
                return Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                });
            }
        }

        // Return an error if authentication fails
        Box::pin(ready(Err(ErrorUnauthorized("Access forbidden"))))
    }
}