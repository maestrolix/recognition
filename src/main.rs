use std::env;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::{get, post}, Router, Json};
use controllers::users::{self, get_users};
use diesel::{Connection, PgConnection, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use models::{NewUser, User};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod models;
pub mod schema;
pub mod controllers;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[utoipa::path(
    get,
    path = "/hello",
    responses(
        (status = 200, description = "Response text 'Hello world!'", body = String)
    )
)]
async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, String::from("Hello world"))
}




#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            hello_world,
            users::create_user,
            users::get_users
        ),
        components(
            schemas(NewUser, User, users::UsersQuery)
        ),
        tags(
            (name = "todo", description = "Todo items management API")
        )
    )]
    struct ApiDoc;


    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/hello", get(hello_world))
        .route("/user", get(get_users).post(users::create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
