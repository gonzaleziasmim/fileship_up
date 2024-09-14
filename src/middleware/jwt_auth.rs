use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub struct AuthenticatedUser(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // Extract the authorization header
        let auth_header = parts.headers.get("Authorization").ok_or((
            StatusCode::UNAUTHORIZED,
            "Missing authorization header".to_string(),
        ))?;

        // Convert header to a string
        let auth_str = auth_header.to_str().map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "Invalid authorization header".to_string(),
            )
        })?;

        // Check if it's a Bearer token
        let token = auth_str
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::BAD_REQUEST, "Invalid token format".to_string()))?;

        // Get the JWT secret key from environment
        let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        // Define validation
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret_key.as_ref()),
            &validation,
        )
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

        // Return the authenticated user's email
        Ok(AuthenticatedUser(token_data.claims.sub))
    }
}
