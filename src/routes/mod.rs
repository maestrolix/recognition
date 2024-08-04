use crate::models::*;
use crate::routes::api::{photos, security, users};
use api::api_router;
use axum::Router;
use pages::template_router;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod api;
pub mod pages;

pub async fn craete_app() -> Router {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            users::post_user,
            users::get_users,
            users::post_user,
            users::delete_user,
            users::get_user,
            users::get_current_user,
            photos::post_photo,
            photos::get_photo,
            photos::get_photos,
            security::sign_in
        ),
        components(
            schemas(NewUser, User, UsersQuery, SignInData, FormUtopia, Photo)
        ),
        tags(
            (name = "users", description = "Управление пользователями"),
            (name = "photos", description = "Управления фотографиями")
        )
    )]
    struct ApiDoc;

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_router().await)
        .nest("/page", template_router().await)
        .nest_service("/storage", ServeDir::new("storage"))
        .layer(CookieManagerLayer::new())
}
