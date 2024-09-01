use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};

use crate::{
    models::*,
    services::albums::{
        create_album, delete_album_by_id, get_album_by_id, get_albums_with_filters,
    },
};

pub async fn router() -> Router {
    Router::new()
        .route("/", post(post_album))
        .route("/:album_id", get(get_album).delete(delete_album))
}

#[utoipa::path(
    post,
    path = "/api/album",
    request_body = NewAlbum,
    responses(
        (status = 201, description = "Create album", body = Album)
    )
)]
pub async fn post_album(Json(new_album): Json<NewAlbum>) -> Json<Album> {
    Json(create_album(new_album).await)
}

#[utoipa::path(
    delete,
    path = "/api/album/{album_id}",
    params(("album_id" = i32, Path, description = "Todo database id")),
    responses(
        (status = 201, description = "Create album")
    )
)]
pub async fn delete_album(Path(album_id): Path<i32>) {
    delete_album_by_id(album_id).await;
}

#[utoipa::path(
    get,
    path = "/api/album/{album_id}",
    params(("album_id" = i32, Path, description = "Id of album")),
    responses(
        (status = 200, description = "Detail info about album", body = Album)
    )
)]
pub async fn get_album(Path(album_id): Path<i32>) -> Json<Album> {
    let album = get_album_by_id(album_id).await;
    Json(album)
}

#[utoipa::path(
    get,
    path = "/api/album",
    responses(
        (status = 200, description = "Detail info about album", body = Vec<Album>)
    )
)]
pub async fn get_albums() -> Json<Vec<Album>> {
    Json(get_albums_with_filters().await)
}
