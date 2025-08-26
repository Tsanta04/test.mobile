use crate::errors::AppError;
use crate::services::Claims;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
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
        let service = self.service.clone();

        // Skip authentication for login and register routes
        if req.path() == "/api/auth/login" || req.path() == "/api/auth/register" {
            return Box::pin(async move {
                let res = service.call(req).await?;
                Ok(res)
            });
        }

        // Get the JWT token from the Authorization header
        let auth_header = req.headers().get("Authorization");
        if auth_header.is_none() {
            return Box::pin(async move {
                Err(AppError::Unauthorized("No Authorization header found".to_string()).into())
            });
        }

        let auth_header = auth_header.unwrap().to_str().unwrap_or("");
        if !auth_header.starts_with("Bearer ") {
            return Box::pin(async move {
                Err(AppError::Unauthorized("Invalid Authorization header format".to_string()).into())
            });
        }

        let token = auth_header.trim_start_matches("Bearer ").trim();
        if token.is_empty() {
            return Box::pin(async move {
                Err(AppError::Unauthorized("Empty token".to_string()).into())
            });
        }

        // Get the JWT secret from the app data
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());

        // Validate the token
        let token_data = match decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(c) => c,
            Err(e) => {
                return Box::pin(async move {
                    Err(AppError::Unauthorized(format!("Invalid token: {}", e)).into())
                });
            }
        };

        // Add the claims to the request extensions
        req.extensions_mut().insert(token_data.claims);

        Box::pin(async move {
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}

