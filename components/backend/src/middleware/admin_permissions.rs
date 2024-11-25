use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
    middleware::Next,
    Extension,
};

use crate::models::User;

pub async fn admin_permissions(
    curr_user: Extension<User>,
    req: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    if curr_user.is_admin {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
