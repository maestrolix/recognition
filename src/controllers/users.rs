use std::env;

use axum::{extract::{Path, Query}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods};
use dotenvy::dotenv;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::{establish_connection, models::{NewUser, User}, schema::users::{email, username}};


#[utoipa::path(
    post,
    path = "/user",
    request_body = NewUser,
    responses(
        (status = 201, description = "Create user account", body = User)
    )
)]
pub async fn create_user(Json(new_user): Json<NewUser>) -> Result<Json<User>, (StatusCode, String)> {
    let user = diesel::insert_into(crate::schema::users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut establish_connection())
        .expect("Error saving new post");
    Ok(Json(user))
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct UsersQuery {
    pub email: Option<String>,
    pub username: Option<String>        
}


#[utoipa::path(
    get,
    path = "/user",
    params(UsersQuery),
    responses(
        (status = 201, description = "Create user account", body = Vec<User>)
    )
)]
pub async fn get_users(Query(params): Query<UsersQuery>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    use crate::schema::users::dsl::*;

    let results = users
        // .filter(email.like(params.email))
        // .filter(username.like(params.username))
        .limit(5)
        .select(User::as_select())
        .load(&mut establish_connection())
        .expect("Error loading posts");
    Ok(Json(results))
}


#[utoipa::path(
    delete,
    path = "/user",
    params(UsersQuery),
    responses(
        (status = 201, description = "Create user account", body = Vec<User>)
    )
)]
pub async fn delete_user(Query(user_id): Query<i32>) -> Result<StatusCode> {
    use crate::schema::users::dsl::*;

    let results = diesel::delete(users.filter(id.eq(user_id)))
    .execute(&mut establish_connection())
    .expect("Error deleting posts");
    Ok(StatusCode::OK)
}