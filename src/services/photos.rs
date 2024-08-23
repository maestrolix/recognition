use axum::body::Bytes;
use axum::http::StatusCode;
use core::f32;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use image::ImageReader;
use pgvector::{Vector, VectorExpressionMethods};
use std::{fs, io::Cursor};

use crate::db_connection::connection;
use crate::fashion_clip::embed::{EmbedImage, EmbedText};
use crate::models::{ListPhoto, NewPhoto, Photo, PhotosFilters};

const UPLOAD_DIR: &str = "storage";

pub async fn create_photo(photo_form: crate::models::PhotoForm, uid: i32) {
    use crate::schema::photos::dsl::*;

    let img_content = photo_form.photo_image.contents.clone();

    // TODO: Вынести логику работы с вектором и организовать на уровне State app
    let embed_image = EmbedImage::new("models/image/model.onnx").unwrap();
    let image: Vec<u8> = img_content.clone().into();
    let embedding_img = match embed_image.encode(image) {
        Ok(d) => d,
        Err(e) => panic!("\n{e}\n"),
    };

    let new_photo = NewPhoto::from_form(&photo_form, embedding_img, uid);

    let photo = diesel::insert_into(photos)
        .values(&new_photo)
        .returning(Photo::as_returning())
        .get_result(&mut connection())
        .expect("Error saving new photo");

    let img_name = format!(
        "{}.{}",
        photo.id,
        photo_form
            .photo_image
            .metadata
            .content_type
            .unwrap()
            .split('/')
            .collect::<Vec<&str>>()[1]
    );
    upload_image(img_content, img_name.clone(), UPLOAD_DIR.to_string()).await;

    diesel::update(photos.find(photo.id))
        .set(path.eq(format!("{}/{}", UPLOAD_DIR, img_name)))
        .execute(&mut connection())
        .unwrap();
}

pub async fn upload_image(content: Bytes, img_name: String, upload_dir: String) {
    let img = ImageReader::new(Cursor::new(content.clone()))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    img.save(format!("{upload_dir}/{img_name}")).unwrap();
}

pub async fn get_photo_by_id(photo_id: i32) -> Option<ListPhoto> {
    use crate::schema::photos::dsl::*;

    match photos
        .find(photo_id)
        .select(ListPhoto::as_select())
        .first(&mut connection())
    {
        Ok(photo) => Some(photo),
        _ => None,
    }
}

pub async fn delete_photo_by_id(photo_id: i32) -> Result<(), StatusCode> {
    use crate::schema::photos::dsl::*;

    let photo: Photo = photos
        .find(photo_id)
        .select(Photo::as_select())
        .first(&mut connection())
        .unwrap();

    fs::remove_file(photo.path).expect("Error remove file");

    diesel::delete(photos.filter(id.eq(photo_id)))
        .execute(&mut connection())
        .expect("Error deleting photo");
    Ok(())
}

pub async fn get_photos_by_filters(filters: PhotosFilters) -> Vec<ListPhoto> {
    use crate::schema::photos;

    let mut query = photos::table.select(ListPhoto::as_select()).into_boxed();

    if let Some(text) = filters.text {
        let embed_text = EmbedText::new(
            "models/text/model.onnx",
            "sentence-transformers/clip-ViT-B-32-multilingual-v1",
        )
        .unwrap();

        let embedding_text = match embed_text.encode(&text) {
            Ok(d) => d.into_iter().flat_map(|i| vec![i]).collect::<Vec<f32>>(),
            Err(e) => panic!("\n{e}\n"),
        };

        query = query.order(photos::embedding.l2_distance(Vector::from(embedding_text)));
    }

    if let Some(qty) = filters.qty {
        query = query.limit(qty.into());
    }
    // query = query.limit(qty_photos.into());

    query.load(&mut connection()).unwrap()
}
