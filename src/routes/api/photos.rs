use axum::extract::Query;
use axum::{extract::Path, http::StatusCode, routing::get, Extension, Json, Router};
use axum_typed_multipart::TypedMultipart;

use crate::{
    models::{ListPhoto, PhotoForm, PhotosFilters, User},
    services::photos::{create_photo, delete_photo_by_id, get_photo_by_id, get_photos_by_filters},
};

pub async fn router() -> Router {
    Router::new()
        .route("/", get(get_photos).post(post_photo))
        .route("/:photo_id", get(get_photo).delete(delete_photo))
        .route("/search", get(search_by_text))
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
pub async fn get_photo(Path(photo_id): Path<i32>) -> Result<Json<ListPhoto>, StatusCode> {
    if let Some(photo) = get_photo_by_id(photo_id).await {
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
    params(PhotosFilters),
    responses(
        (status = 200, description = "Get all photos of user", body = Vec<Photo>)
    )
)]
pub async fn get_photos(Query(filters): Query<PhotosFilters>) -> Json<Vec<ListPhoto>> {
    Json(get_photos_by_filters(filters).await)
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
pub async fn delete_photo(Path(photo_id): Path<i32>) -> StatusCode {
    match delete_photo_by_id(photo_id).await {
        Ok(()) => StatusCode::OK,
        Err(code) => code,
    }
}

#[utoipa::path(
    get,
    path = "/api/photo/search",
    tag = "photos",
    params(PhotosFilters),
    responses(
        (status = 200, description = "Search image by text", body = Vec<ListPhoto>)
    )
)]
pub async fn search_by_text(Query(filters): Query<PhotosFilters>) -> Json<Vec<ListPhoto>> {
    Json(get_photos_by_filters(filters).await)
}
