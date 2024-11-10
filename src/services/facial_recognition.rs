use diesel::{
    result, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};
use image::{io::Reader as ImageReader, DynamicImage, ImageError, RgbImage};
use pgvector::{Vector, VectorExpressionMethods};
use reqwest::multipart;
use serde::{Deserialize, Serialize};

use crate::db_connection::connection;
use crate::models::{Face, NewFace, NewPerson, NewPhoto, Photo, PhotoForm};

use std::io::Cursor;
use thiserror::Error;
use tokio::io;

const UPLOAD_DIR_IMAGES: &str = "storage/images";
const UPLOAD_DIR_FACES: &str = "storage/faces";

#[derive(Error, Debug)]
pub enum FaceError {
    #[error("ORM request error")]
    DieselError(#[from] result::Error),

    #[error("StdIO error: {0}")]
    StdIO(#[from] io::Error),

    #[error("Image error: {0}")]
    ImageError(#[from] ImageError),

    #[error("unknown data store error")]
    Unknown,
    // Делал для OPTION
    // #[error("")]
    // Infallible(#[from] std::convert::Infallible),
}

pub async fn create_photo(photo_form: PhotoForm, uid: i32) -> Result<(), FaceError> {
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
            photos::embedding.eq(Vector::from(clip_visual_from_ml(&file_path).await)),
        ))
        .returning(Photo::as_returning())
        .get_result(&mut connection())?;

    cut_faces_and_save(photo).await.unwrap();
    Ok(())
}

pub async fn cut_faces_and_save(photo: Photo) -> Result<(), FaceError> {
    use crate::schema::{faces, persons};

    let raw_image = image::open(photo.path.as_ref().unwrap())?.to_rgb8();
    let faces = faces_recognition_from_ml(&photo.path.unwrap()).await;

    for face in faces {
        let db_face: Face = diesel::insert_into(faces::table)
            .values(&NewFace { photo_id: photo.id })
            .returning(Face::as_returning())
            .get_result(&mut connection())?;

        let image_face_path = format!("{UPLOAD_DIR_FACES}/{}.jpeg", db_face.id);
        let pg_vector_embedding = Vector::from(face.embedding);

        cut_image(&raw_image, &face.bounding_box).save(&image_face_path)?;

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
                faces::bbox.eq(Some(face.bounding_box.to_pg_array())),
                faces::embedding.eq(Some(pg_vector_embedding)),
                faces::person_id.eq(person_id),
            ))
            .execute(&mut connection())?;
    }

    Ok(())
}

fn cut_image(image: &RgbImage, bb: &BoundingBox) -> DynamicImage {
    let rect_width = bb.x_br - bb.x_tl;
    let rect_height = bb.y_br - bb.y_tl;

    DynamicImage::from(
        image::imageops::crop_imm(
            image,
            bb.x_tl as u32,
            bb.y_tl as u32,
            rect_width as u32,
            rect_height as u32,
        )
        .to_image(),
    )
}

#[derive(Debug, Deserialize, Serialize)]
struct BoundingBox {
    #[serde(rename = "x1")]
    pub x_tl: f32,
    #[serde(rename = "y1")]
    pub y_tl: f32,
    #[serde(rename = "x2")]
    pub x_br: f32,
    #[serde(rename = "y2")]
    pub y_br: f32,
}

impl BoundingBox {
    pub fn to_pg_array(self) -> Vec<Option<i32>> {
        vec![self.x_tl, self.y_tl, self.x_br, self.y_br]
            .iter()
            .map(|e| Some(*e as i32))
            .collect::<Vec<Option<i32>>>()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct DetectedFace {
    #[serde(rename = "boundingBox")]
    bounding_box: BoundingBox,
    embedding: Vec<f32>,
    score: f32,
}

async fn faces_recognition_from_ml(path: &str) -> Vec<DetectedFace> {
    let form_data = multipart::Form::new().file("image", path).await.unwrap();

    let response_body = reqwest::Client::new()
        .post("http://0.0.0.0:5005/recognition-faces")
        .multipart(form_data)
        .send()
        .await
        .expect("send")
        .text()
        .await
        .unwrap();

    serde_json::from_str(&response_body).unwrap()
}

pub async fn clip_textual_from_ml(text: String) -> Vec<f32> {
    let form_data = multipart::Form::new().text("text", text);

    let response_body = reqwest::Client::new()
        .post("http://0.0.0.0:5005/clip-textual")
        .multipart(form_data)
        .send()
        .await
        .expect("send")
        .text()
        .await
        .unwrap();

    serde_json::from_str(&response_body).unwrap()
}

pub async fn clip_visual_from_ml(path: &str) -> Vec<f32> {
    let form_data = multipart::Form::new().file("image", path).await.unwrap();

    let response_body = reqwest::Client::new()
        .post("http://0.0.0.0:5005/clip-visual")
        .multipart(form_data)
        .send()
        .await
        .expect("send")
        .text()
        .await
        .unwrap();

    serde_json::from_str(&response_body).unwrap()
}
