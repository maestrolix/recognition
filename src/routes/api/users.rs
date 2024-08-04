use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use crate::{
    models::*,
    services::users::{create_user, delete_user_by_id, get_user_by_id, get_users_with_filters},
};

pub async fn router() -> Router {
    Router::new()
        .route("/:user_id", get(get_user).delete(delete_user))
        .route("/", get(get_users).post(post_user))
        .route("/current_user", get(get_current_user))
}

#[utoipa::path(
    post,
    path = "/api/user",
    tag = "users",
    request_body = NewUser,
    responses(
        (status = 201, description = "Create user account", body = User)
    )
)]
pub async fn post_user(Json(new_user): Json<NewUser>) -> Result<Json<User>, (StatusCode, String)> {
    let user = create_user(new_user).await;
    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "/api/user",
    tag = "users",
    params(UsersQuery),
    responses(
        (status = 201, description = "Create user account", body = Vec<User>)
    )
)]
pub async fn get_users(
    Query(params): Query<UsersQuery>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = get_users_with_filters(params).await;
    Ok(Json(users))
}

#[utoipa::path(
    delete,
    path = "/api/user/{user_id}",
    tag = "users",
    params(("user_id" = i32, Path, description = "Todo database id")),
    responses(
        (status = 201, description = "Create user account", body = StatusCode)
    )
)]
pub async fn delete_user(Path(user_id): Path<i32>) -> StatusCode {
    delete_user_by_id(user_id).await;
    StatusCode::OK
}

#[utoipa::path(
    get,
    path = "/api/user/{user_id}",
    tag = "users",
    params(("user_id" = i32, Path, description = "Id of user")),
    responses(
        (status = 200, description = "Detail info about user", body = User)
    )
)]
pub async fn get_user(Path(user_id): Path<i32>) -> Result<Json<User>, (StatusCode, String)> {
    let user = get_user_by_id(user_id).await;
    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "/api/user/current_user",
    tag = "users",
    responses(
        (status = 200, description = "Current user", body = User)
    )
)]
pub async fn get_current_user(
    curr_user: Extension<User>,
) -> Result<Json<User>, (StatusCode, String)> {
    Ok(Json(curr_user.0.clone()))
}
