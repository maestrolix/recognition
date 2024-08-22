use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    middleware,
    routing::get,
    Json, Router,
};

use crate::{
    errors::Error,
    middleware::admin_permissions,
    models::*,
    services::users::{create_user, delete_user_by_id, get_user_by_id, get_users_with_filters},
};

pub async fn router() -> Router {
    Router::new()
        .route(
            "/:user_id",
            get(get_user)
                .delete(delete_user)
                .layer(middleware::from_fn(admin_permissions::admin_permissions)),
        )
        .route(
            "/",
            get(get_users)
                .post(post_user)
                .layer(middleware::from_fn(admin_permissions::admin_permissions)),
        )
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
pub async fn post_user(Json(new_user): Json<NewUser>) -> Json<User> {
    let user = create_user(new_user).await;
    Json(user)
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
pub async fn get_users(Query(params): Query<UsersQuery>) -> Json<Vec<User>> {
    let users = get_users_with_filters(params).await;
    Json(users)
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
pub async fn delete_user(Path(user_id): Path<i32>) {
    delete_user_by_id(user_id).await;
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
pub async fn get_user(Path(user_id): Path<i32>) -> Result<Json<User>, Error> {
    match get_user_by_id(user_id).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Error::new("User not found", StatusCode::NOT_FOUND)),
    }
}

#[utoipa::path(
    get,
    path = "/api/user/current_user",
    tag = "users",
    responses(
        (status = 200, description = "Current user", body = User)
    )
)]
pub async fn get_current_user(Extension(curr_user): Extension<User>) -> Json<User> {
    Json(curr_user.clone())
}
