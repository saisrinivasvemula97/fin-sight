use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user id
    pub email: String,
    pub exp: usize,   // expiration time
    pub iat: usize,   // issued at time
}

// Authentication request models
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: AuthUser,
}

#[derive(Debug, Serialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
}

// Password hashing utilities (simplified for now)
pub fn hash_password(password: &str) -> String {
    // In a real implementation, use proper hashing like argon2
    // For now, we'll use a simple approach for testing
    format!("hashed_{}", password)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    // In a real implementation, use proper verification
    hash == &format!("hashed_{}", password)
}

// JWT utilities
pub fn create_jwt(user_id: &str, email: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_exp = true;
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation
    )
    .map(|token_data| token_data.claims)
}

// Generate a new UUID
pub fn generate_uuid() -> String {
    Utc::now().timestamp_nanos().to_string()
}