use anyhow::Result;
use axum::http::StatusCode;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use pgvector::{Vector, VectorExpressionMethods};
use tokio::fs;

use crate::db_connection::connection;
use crate::models::{ListPhoto, Photo, PhotosFilters};
use crate::services::facial_recognition::clip_textual_from_ml;

pub async fn get_photo_by_id(photo_id: i32) -> Result<ListPhoto> {
    use crate::schema::photos::dsl::*;
    Ok(photos
        .find(photo_id)
        .select(ListPhoto::as_select())
        .first(&mut connection())?)
}

pub async fn delete_photo_by_id(photo_id: i32) -> Result<(), StatusCode> {
    use crate::schema::photos::dsl::*;

    let photo: Photo = photos
        .find(photo_id)
        .select(Photo::as_select())
        .first(&mut connection())
        .unwrap();

    fs::remove_file(photo.path.unwrap())
        .await
        .expect("Error remove file");

    diesel::delete(photos.filter(id.eq(photo_id)))
        .execute(&mut connection())
        .expect("Error deleting photo");
    Ok(())
}

pub async fn get_photos_by_filters(filters: PhotosFilters) -> Vec<ListPhoto> {
    use crate::schema::photos;

    let mut query = photos::table.select(ListPhoto::as_select()).into_boxed();

    if let Some(text) = filters.text {
        let pg_vector_embedding = Vector::from(clip_textual_from_ml(text).await.unwrap());
        query = query.order(photos::embedding.cosine_distance(pg_vector_embedding));
    }

    if let Some(qty) = filters.qty {
        query = query.limit(qty.into());
    }

    query.load(&mut connection()).unwrap()
}
