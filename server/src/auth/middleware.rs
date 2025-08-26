use crate::auth::jwt::{validate_token, Claims};
use crate::error::AppError;
use crate::models::user::UserRole;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

// Define a struct to hold the authentication middleware configuration
pub struct Auth {
    pub roles: Vec<UserRole>,
}

impl Auth {
    pub fn new(roles: Vec<UserRole>) -> Self {
        Self { roles }
    }
}

// Implement the Transform trait for Auth
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
            roles: self.roles.clone(),
        }))
    }
}

// Define the middleware service
pub struct AuthMiddleware<S> {
    service: Rc<S>,
    roles: Vec<UserRole>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
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
        let service = Rc::clone(&self.service);
        let roles = self.roles.clone();

        Box::pin(async move {
            // Extract the token from the Authorization header
            let auth_header = req
                .headers()
                .get("Authorization")
                .ok_or_else(|| {
                    let err = AppError::Unauthorized("Missing Authorization header".to_string());
                    Error::from(err)
                })?
                .to_str()
                .map_err(|_| {
                    let err = AppError::Unauthorized("Invalid Authorization header".to_string());
                    Error::from(err)
                })?;

            // Check if the header starts with "Bearer "
            if !auth_header.starts_with("Bearer ") {
                let err = AppError::Unauthorized("Invalid Authorization header format".to_string());
                return Err(Error::from(err));
            }

            // Extract the token
            let token = &auth_header[7..];

            // Validate the token
            let token_data = validate_token(token).map_err(Error::from)?;

            // Check if the user has the required role
            if !roles.is_empty() && !roles.contains(&token_data.claims.role) {
                let err = AppError::Forbidden("Insufficient permissions".to_string());
                return Err(Error::from(err));
            }

            // Add the claims to the request extensions
            req.extensions_mut().insert(token_data.claims);

            // Call the next service
            service.call(req).await
        })
    }
}

// Extension trait to easily extract claims from the request
pub trait ClaimsExtractor {
    fn claims(&self) -> Result<&Claims, AppError>;
}

impl ClaimsExtractor for actix_web::HttpRequest {
    fn claims(&self) -> Result<&Claims, AppError> {
        self.extensions()
            .get::<Claims>()
            .ok_or_else(|| AppError::Unauthorized("User not authenticated".to_string()))
    }
}

