use crate::models::*;
use crate::routes::api::{albums, photos, security, users};
use api::api_router;
use axum::extract::DefaultBodyLimit;
use axum::Router;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, services::ServeDir};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod api;

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
            photos::delete_photo,
            photos::search_by_text,

            albums::get_album,
            albums::delete_album,
            albums::post_album,
            albums::get_albums,

            security::sign_in
        ),
        components(
            schemas(NewUser, User, UsersQuery, SignInData, PhotoFormUtopia, Photo, ListPhoto, Album, NewAlbum)
        ),
        tags(
            (name = "users", description = "Управление пользователями"),
            (name = "albums", description = "Управление альбомами"),
            (name = "photos", description = "Управления фотографиями")
        )
    )]
    struct ApiDoc;

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_router().await)
        .nest_service("/storage", ServeDir::new("storage"))
        .layer(CookieManagerLayer::new())
        .layer(DefaultBodyLimit::max(100000000))
        .layer(CorsLayer::permissive())
}
