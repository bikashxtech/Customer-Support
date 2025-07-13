use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use std::env;
use once_cell::sync::Lazy;
use axum::{
    async_trait,
    extract::{FromRequestParts},
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData, errors::Error as JwtError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

// Role checker
impl Claims {
    pub fn is_agent(&self) -> bool {
        self.role == "Agent"
    }

    pub fn is_customer(&self) -> bool {
        self.role == "Customer"
    }
}

static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
});

pub fn generate_token(user_id: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, (StatusCode, String)> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".into()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid Bearer token format".into()))?;

        decode_token(token).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".into()))
    }
}

pub fn decode_token(token: &str) -> Result<Claims, JwtError> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let decoded: TokenData<Claims> = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(decoded.claims)
}
