use axum::body::Bytes;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use image::ImageReader;
use std::io::Cursor;

use crate::db_connection::connection;
use crate::models::{NewPhoto, Photo};

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
        "{}_{}.{}",
        photo.user_id,
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
