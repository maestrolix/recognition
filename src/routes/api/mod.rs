use axum::{middleware, routing::post, Router};

use crate::middleware::authorize;

pub mod albums;
pub mod photos;
pub mod security;
pub mod users;

pub async fn api_router() -> Router {
    Router::new()
        .nest(
            "/user",
            users::router()
                .await
                .layer(middleware::from_fn(authorize::authorize)),
        )
        .nest(
            "/album",
            albums::router()
                .await
                .layer(middleware::from_fn(authorize::authorize)),
        )
        .nest(
            "/photo",
            photos::router().await, // .layer(middleware::from_fn(authorize::authorize)),
        )
        .route("/signin", post(security::sign_in))
}
