use axum::{extract::Json, http::StatusCode};
use tower_cookies::{Cookie, Cookies};

use crate::{
    auth::{encode_jwt, verify_password},
    models::*,
    services::users::get_user_by_email,
};
use axum::{routing::post, Router};

pub async fn router() -> Router {
    Router::new().route("/login/api", post(sign_in))
}

#[utoipa::path(
    post,
    path = "/api/signin",
    responses(
        (status = 200, description = "Sign user", body = SignInData)
    )
)]
pub async fn sign_in(
    cookies: Cookies,
    Json(user_data): Json<SignInData>,
) -> Result<StatusCode, StatusCode> {
    let user = match get_user_by_email(&user_data.email).await {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if !verify_password(&user_data.password, &user.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = encode_jwt(user.email).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    cookies.add(Cookie::new("token", token));
    Ok(StatusCode::OK)
}
