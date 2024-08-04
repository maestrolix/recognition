use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::{
    models::*,
    services::albums::{create_album, delete_album_by_id, get_album_by_id},
};

pub async fn router() -> Router {
    Router::new()
        .route("/album", post(post_album))
        .route("/album/:album_id", get(get_album).delete(delete_album))
}

#[utoipa::path(
    post,
    path = "/album",
    request_body = NewAlbum,
    responses(
        (status = 201, description = "Create album", body = Album)
    )
)]
pub async fn post_album(
    Json(new_album): Json<NewAlbum>,
) -> Result<Json<Album>, (StatusCode, String)> {
    let album = create_album(new_album).await;
    Ok(Json(album))
}

#[utoipa::path(
    delete,
    path = "/album/{album_id}",
    params(("album_id" = i32, Path, description = "Todo database id")),
    responses(
        (status = 201, description = "Create album", body = StatusCode)
    )
)]
pub async fn delete_album(Path(album_id): Path<i32>) -> StatusCode {
    delete_album_by_id(album_id).await;
    StatusCode::OK
}

#[utoipa::path(
    get,
    path = "/album/{album_id}",
    params(("album_id" = i32, Path, description = "Id of album")),
    responses(
        (status = 200, description = "Detail info about album", body = Album)
    )
)]
pub async fn get_album(Path(album_id): Path<i32>) -> Result<Json<Album>, (StatusCode, String)> {
    let album = get_album_by_id(album_id).await;
    Ok(Json(album))
}
