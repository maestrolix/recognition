use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use image::{io::Reader as ImageReader, DynamicImage, RgbImage};
use pgvector::{Vector, VectorExpressionMethods};
use reqwest::multipart;
use serde::{Deserialize, Serialize};

use crate::db_connection::connection;
use crate::errors::CreatePhotoError;
use crate::models::{Face, NewFace, NewPerson, NewPhoto, Photo, PhotoForm};

use std::io::Cursor;

const UPLOAD_DIR_IMAGES: &str = "storage/images";
const UPLOAD_DIR_FACES: &str = "storage/faces";

pub async fn create_photo(photo_form: PhotoForm, uid: i32) -> Result<(), CreatePhotoError> {
    use crate::schema::photos;

    let mut photo: Photo = diesel::insert_into(photos::table)
        .values(NewPhoto::from_form(&photo_form, uid))
        .returning(Photo::as_returning())
        .get_result(&mut connection())?;

    let file_content = photo_form.photo_image.contents.clone();

    let file_path = format!("{UPLOAD_DIR_IMAGES}/{}.jpeg", photo.id);

    let dyn_img = ImageReader::new(Cursor::new(file_content))
        .with_guessed_format()?
        .decode()?;

    dyn_img.save(&file_path)?;

    photo = diesel::update(photos::table.find(photo.id))
        .set((
            photos::path.eq(&file_path),
            photos::embedding.eq(Vector::from(clip_visual_from_ml(&file_path).await?)),
        ))
        .returning(Photo::as_returning())
        .get_result(&mut connection())?;

    cut_faces_and_save(photo).await?;
    Ok(())
}

pub async fn cut_faces_and_save(photo: Photo) -> Result<(), CreatePhotoError> {
    use crate::schema::{faces, persons};

    let raw_image = image::open(photo.path.as_ref().unwrap())?.to_rgb8();
    let faces = faces_recognition_from_ml(&photo.path.unwrap()).await?;

    for face in faces {
        let db_face: Face = diesel::insert_into(faces::table)
            .values(&NewFace { photo_id: photo.id })
            .returning(Face::as_returning())
            .get_result(&mut connection())?;

        let image_face_path = format!("{UPLOAD_DIR_FACES}/{}.jpeg", db_face.id);
        let pg_vector_embedding = Vector::from(face.embedding);

        cut_image(&raw_image, &face.bbox).save(&image_face_path)?;

        let nearest_face: Option<Face> = faces::table
            .select(Face::as_select())
            .filter(faces::embedding.is_not_null())
            .filter(faces::person_id.is_not_null())
            .filter(
                faces::embedding
                    .cosine_distance(pg_vector_embedding.clone())
                    .le(0.5),
            )
            .order(faces::embedding.cosine_distance(pg_vector_embedding.clone()))
            .first(&mut connection())
            .optional()?;

        let person_id = match nearest_face {
            Some(person_face) => person_face.person_id.unwrap(),
            None => {
                let new_person = NewPerson {
                    title: "Unknown".to_string(),
                    avatar: image_face_path,
                };
                diesel::insert_into(persons::table)
                    .values(&new_person)
                    .returning(persons::id)
                    .get_result::<i32>(&mut connection())?
            }
        };

        diesel::update(faces::table.find(db_face.id))
            .set((
                faces::path.eq(format!("{UPLOAD_DIR_FACES}/{}.jpeg", db_face.id)),
                faces::bbox.eq(Some(face.bbox.map(|el| Some(el as i32)).to_vec())),
                faces::embedding.eq(Some(pg_vector_embedding)),
                faces::person_id.eq(person_id),
            ))
            .execute(&mut connection())?;
    }

    Ok(())
}

fn cut_image(image: &RgbImage, bb: &[f32; 4]) -> DynamicImage {
    let (x_tl, y_tl, x_br, y_br) = (bb[0], bb[1], bb[2], bb[3]);

    let rect_width = x_br - x_tl;
    let rect_height = y_br - y_tl;

    DynamicImage::from(
        image::imageops::crop_imm(
            image,
            x_tl as u32,
            y_tl as u32,
            rect_width as u32,
            rect_height as u32,
        )
        .to_image(),
    )
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecognizedFaceOutput {
    pub score: f32,
    pub bbox: [f32; 4],
    pub landmarks: [(f32, f32); 5],
    pub embedding: Vec<f32>,
}

async fn faces_recognition_from_ml(
    path: &str,
) -> Result<Vec<RecognizedFaceOutput>, CreatePhotoError> {
    let form_data = multipart::Form::new().file("image", path).await?;

    let response_body = reqwest::Client::new()
        .post("http://0.0.0.0:3003/recognition-faces")
        .multipart(form_data)
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&response_body)?)
}

pub async fn clip_textual_from_ml(text: String) -> Result<Vec<f32>, CreatePhotoError> {
    let response_body = reqwest::Client::new()
        .post("http://0.0.0.0:3003/clip-textual")
        .query(&[("text", &text)])
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&response_body)?)
}

pub async fn clip_visual_from_ml(path: &str) -> Result<Vec<f32>, CreatePhotoError> {
    let form_data = multipart::Form::new().file("image", path).await?;

    let response_body = reqwest::Client::new()
        .post("http://0.0.0.0:3003/clip-visual")
        .multipart(form_data)
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&response_body)?)
}
