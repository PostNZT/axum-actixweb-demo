use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const JWT_SECRET: &str = "your-secret-key-here";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub username: String,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: Uuid, username: String, email: String) -> Self {
        let now = Utc::now();
        let expires_at = now + Duration::hours(24);

        Self {
            sub: user_id.to_string(),
            username,
            email,
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn hash_password(password: &str) -> Result<String> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let is_valid = verify(password, hash)?;
    Ok(is_valid)
}

pub fn create_jwt(claims: &Claims) -> Result<String> {
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )?;
    Ok(token)
}

pub fn validate_jwt(token: &str) -> Result<TokenData<Claims>> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data)
}