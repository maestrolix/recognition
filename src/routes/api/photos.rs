use axum::{extract::Path, http::StatusCode, routing::get, Extension, Json, Router};
use axum_typed_multipart::TypedMultipart;

use crate::{
    models::{Photo, PhotoForm, User},
    services::photos::{create_photo, get_photo_by_id, get_photos_by_filters},
};

pub async fn router() -> Router {
    Router::new()
        .route("/", get(get_photos).post(post_photo))
        .route("/:photo_id", get(get_photo))
}

#[utoipa::path(
    get,
    path = "/api/photo/{photo_id}",
    tag = "photos",
    params(("photo_id" = i32, Path, description = "Photo id")),
    responses(
        (status = 200, description = "Get photo info", body = Photo)
    )
)]
pub async fn get_photo(
    curr_user: Extension<User>,
    Path(photo_id): Path<i32>,
) -> Result<Json<Photo>, StatusCode> {
    if let Some(photo) = get_photo_by_id(photo_id, curr_user.id).await {
        return Ok(Json(photo));
    } else {
        return Err(StatusCode::FORBIDDEN);
    }
}

#[utoipa::path(
    post,
    path = "/api/photo",
    tag = "photos",
    request_body(content_type="multipart/form-data", content=FormUtopia),
    responses(
        (status = 201, description = "Add new photo", body = Photo)
    )
)]
pub async fn post_photo(
    curr_user: Extension<User>,
    photo_form: TypedMultipart<PhotoForm>,
) -> Result<StatusCode, StatusCode> {
    create_photo(photo_form.0, curr_user.id).await;
    Ok(StatusCode::OK)
}

#[utoipa::path(
    get,
    path = "/api/photo",
    tag = "photos",
    responses(
        (status = 200, description = "Get all photos of user", body = Vec<Photo>)
    )
)]
pub async fn get_photos(curr_user: Extension<User>) -> Json<Vec<Photo>> {
    Json(get_photos_by_filters(curr_user.id).await)
}
