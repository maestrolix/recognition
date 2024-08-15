use axum::{extract::Path, http::StatusCode, routing::get, Extension, Json, Router};
use axum_typed_multipart::TypedMultipart;

use crate::models::ListPhoto;

use crate::{
    models::{PhotoForm, User},
    services::photos::{
        create_photo, delete_photo_by_id, get_photo_by_id, get_photos_by_filters,
        search_by_text_service,
    },
};

pub async fn router() -> Router {
    Router::new()
        .route("/", get(get_photos).post(post_photo))
        .route("/:photo_id", get(get_photo).delete(delete_photo))
        .route("/search/:text", get(search_by_text))
}

#[utoipa::path(
    get,
    path = "/api/photo/{photo_id}",
    tag = "photos",
    params(("photo_id" = i32, Path, description = "Photo id")),
    responses(
        (status = 200, description = "Get photo info", body = ListPhoto)
    )
)]
pub async fn get_photo(
    curr_user: Extension<User>,
    Path(photo_id): Path<i32>,
) -> Result<Json<ListPhoto>, StatusCode> {
    if let Some(photo) = get_photo_by_id(photo_id, curr_user.id).await {
        Ok(Json(photo))
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

#[utoipa::path(
    post,
    path = "/api/photo",
    tag = "photos",
    request_body(content_type="multipart/form-data", content=PhotoFormUtopia),
    responses(
        (status = 201, description = "Add new photo", body = Photo)
    )
)]
pub async fn post_photo(curr_user: Extension<User>, photo_form: TypedMultipart<PhotoForm>) {
    create_photo(photo_form.0, curr_user.id).await;
}

#[utoipa::path(
    get,
    path = "/api/photo",
    tag = "photos",
    responses(
        (status = 200, description = "Get all photos of user", body = Vec<Photo>)
    )
)]
pub async fn get_photos(curr_user: Extension<User>) -> Json<Vec<ListPhoto>> {
    Json(get_photos_by_filters(curr_user.id).await)
}

#[utoipa::path(
    delete,
    path = "/api/photo/{photo_id}",
    tag = "photos",
    params(("photo_id" = i32, Path, description = "Todo database id")),
    responses(
        (status = 201, description = "Delete photo")
    )
)]
pub async fn delete_photo(Path(photo_id): Path<i32>, curr_user: Extension<User>) -> StatusCode {
    match delete_photo_by_id(photo_id, curr_user.0).await {
        Ok(()) => StatusCode::OK,
        Err(code) => code,
    }
}

#[utoipa::path(
    get,
    path = "/api/photo/search/{text}",
    tag = "photos",
    params(
        ("text", description = "Text to find image")
    ),
    responses(
        (status = 200, description = "Search image by text", body = Photo)
    )
)]
pub async fn search_by_text(Path(text): Path<String>, Extension(curr_user): Extension<User>) -> Json<ListPhoto> {
    Json(search_by_text_service(text, curr_user.id).await)
}
