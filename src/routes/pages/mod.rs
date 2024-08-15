use axum::Router;
use tower_http::services::ServeDir;

pub mod gallery;
pub mod security;

pub async fn template_router() -> Router {
    Router::new()
        .nest("/login", security::router().await)
        .nest("/gallery", gallery::router().await)
        .nest_service("/", ServeDir::new("static"))
}
