use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
    middleware::Next,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use crate::{middleware::errors::Error, services::users::get_user_by_email};

#[derive(Serialize, Deserialize)]
pub struct Cliams {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hash = hash(password, DEFAULT_COST)?;
    Ok(hash)
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let jwt_token: String = "randomstring".to_string();

    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;

    let claim = Cliams { iat, exp, email };
    let secret = jwt_token.clone();

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Cliams>, StatusCode> {
    let secret = "randomstring".to_string();

    let result: Result<TokenData<Cliams>, StatusCode> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}

pub async fn authorize(
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, Error> {
    let token = match cookies.get("token") {
        Some(token) => token.value().to_string(),
        None => return Err(Error::new("Token not found", StatusCode::UNAUTHORIZED)),
    };

    let token_data = match decode_jwt(token) {
        Ok(data) => data,
        Err(_) => {
            return Err(Error::new(
                "Unable to decode token",
                StatusCode::UNAUTHORIZED,
            ))
        }
    };

    let current_user = get_user_by_email(&token_data.claims.email).await.unwrap();

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}
