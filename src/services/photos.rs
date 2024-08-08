use axum::body::Bytes;
use axum::http::StatusCode;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use image::ImageReader;
use std::{fs, io::Cursor};

use crate::db_connection::connection;
use crate::fashion_clip::embed::{EmbedImage, EmbedText};
use crate::models::{NewPhoto, Photo, User};
use crate::utils::cosine_similarity;

const UPLOAD_DIR: &str = "storage";

pub async fn create_photo(photo_form: crate::models::PhotoForm, uid: i32) {
    use crate::schema::photos::dsl::*;

    let new_photo = NewPhoto::from_form(&photo_form, uid);

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
            .split("/")
            .collect::<Vec<&str>>()[1]
    );
    let img_content = photo_form.photo_image.contents.clone();
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

pub async fn get_photo_by_id(photo_id: i32, uid: i32) -> Option<Photo> {
    use crate::schema::photos::dsl::*;

    match photos
        .find(photo_id)
        .filter(user_id.eq(uid))
        .select(Photo::as_select())
        .first(&mut connection())
    {
        Ok(photo) => Some(photo),
        _ => None,
    }
}

pub async fn get_photos_by_filters(uid: i32) -> Vec<Photo> {
    use crate::schema::photos::dsl::*;

    photos
        .filter(user_id.eq(uid))
        .select(Photo::as_select())
        .load(&mut connection())
        .unwrap()
}

pub async fn delete_photo_by_id(photo_id: i32, curr_user: User) -> Result<(), StatusCode> {
    use crate::schema::photos::dsl::*;

    let photo: Photo = photos
        .find(photo_id)
        .select(Photo::as_select())
        .first(&mut connection())
        .unwrap();

    if curr_user.id == photo.user_id || curr_user.is_admin {
        fs::remove_file(photo.path).expect("Error remove file");

        diesel::delete(photos.filter(id.eq(photo_id)))
            .execute(&mut connection())
            .expect("Error deleting photo");
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

pub async fn search_by_text_service(text: String, curr_user: User) -> Vec<f32> {
    let embed_text = EmbedText::new(
        "models/text/model.onnx",
        "sentence-transformers/clip-ViT-B-32-multilingual-v1",
    )
    .unwrap();
    let embed_image = EmbedImage::new("models/image/model.onnx").unwrap();

    let embedding_text = match embed_text.encode(&text) {
        Ok(d) => d.into_iter().flat_map(|i| vec![i]).collect::<Vec<f32>>(),
        Err(e) => panic!("\n{e}\n"),
    };
    println!("\n{:?}", embedding_text);

    let photos_db = get_photos_by_filters(curr_user.id).await;

    let mut img_embeddings: Vec<(Vec<f32>, String)> = Vec::new();

    for photo in photos_db {
        
        let image = std::fs::read(&photo.path).unwrap();
        let embedding_img = match embed_image.encode(image) {
            Ok(d) => d,
            Err(e) => panic!("\n{e}\n"),
        };
        img_embeddings.push((embedding_img, photo.path));
    }

    for img_emb in img_embeddings {
        let cos_sim = cosine_similarity(&img_emb.0, &embedding_text, false);
        println!("Text: {}", text);
        println!("Score: {}", cos_sim);
        println!("Path: {}", img_emb.1);
    }

    return embedding_text;
}
