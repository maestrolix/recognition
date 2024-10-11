use axum::http::StatusCode;
use core::f32;
use diesel::result::Error;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{DynamicImage, RgbImage};
use pgvector::{Vector, VectorExpressionMethods};
use std::path::Path;
use std::process;
use std::{fs, io::Cursor};

use crate::db_connection::connection;
use crate::models::{Face, ListPhoto, NewFace, NewPerson, NewPhoto, Person, Photo, PhotosFilters};
use crate::photo_processing::clip::embed::{EmbedImage, EmbedText};
use crate::photo_processing::ultraface::post_processor::Bbox;
use crate::photo_processing::ultraface::ultra_image::{draw_bboxes_on_image, UltraImage};
use crate::photo_processing::ultraface::ultra_predictor::{
    UltraPredictor, ULTRA_INPUT_HEIGHT, ULTRA_INPUT_WIDTH,
};
use crate::schema::users::avatar;
use crate::utils::cosine_similarity;

const UPLOAD_DIR: &str = "storage/photos";

pub async fn create_photo(photo_form: crate::models::PhotoForm, uid: i32) {
    use crate::schema::photos::dsl::*;

    let dyn_img = ImageReader::new(Cursor::new(photo_form.photo_image.contents.clone()))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    // TODO: Вынести логику работы с вектором и организовать на уровне State app
    let embed_image = EmbedImage::new("models/clip/image/model.onnx").unwrap();
    let embedding_img = match embed_image.encode(dyn_img.clone()) {
        Ok(d) => d,
        Err(e) => panic!("\n{e}\n"),
    };

    let new_photo = NewPhoto::from_form(&photo_form, embedding_img, uid);

    // TODO: Вынести логику создания подключения из функции выше
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

    // Если сохраняем в том же формате, может возьмем картинку из `photo_form.photo_image.contents`? И не надо буде енкодить
    dyn_img.save(format!("{UPLOAD_DIR}/{img_name}")).unwrap();

    diesel::update(photos.find(photo.id))
        .set(path.eq(format!("{UPLOAD_DIR}/{img_name}")))
        .execute(&mut connection())
        .unwrap();

    // ultraface_logic(photo.id).await;
}

async fn facenet_logic(photo_id: i32) {
    use crate::schema::{faces, persons, photos};

    let photo: Photo = photos::table
        .select(Photo::as_select())
        .find(photo_id)
        .get_result(&mut connection())
        .unwrap();

    let img_path = Path::new(&photo.path);
}

async fn ultraface_logic(photo_id: i32) {
    use crate::schema::{faces, persons, photos};

    let photo: Photo = photos::table
        .select(Photo::as_select())
        .find(photo_id)
        .get_result(&mut connection())
        .unwrap();

    let img_path = Path::new(&photo.path);

    let ultra_model_path = Path::new("models/ultraface/version-RFB-640.onnx");

    let ultra_predictor = UltraPredictor::new(ultra_model_path).unwrap_or_else(|ort_err| {
        println!("Problem creating ultra onnx session: {}", ort_err);
        process::exit(1)
    });
    let mut ultra_image = UltraImage::new(img_path).unwrap();
    let (img_width, img_height) = (
        ultra_image.raw_image.width(),
        ultra_image.raw_image.height(),
    );

    let ultra_output = &ultra_predictor.run(&ultra_image.image).unwrap();

    let embed_image = EmbedImage::new("models/clip/image/model.onnx").unwrap();

    ultra_image
        .draw_bboxes(
            ultra_output.bbox_with_confidences.clone(),
            Path::new("res/"),
        )
        .unwrap();

    for (bbox, _score) in &ultra_output.bbox_with_confidences {
        let face = cut_image(&ultra_image.image, bbox);

        let embedding_face = match embed_image.encode(face.clone()) {
            Ok(d) => d,
            Err(e) => panic!("\n{e}\n"),
        };
        let nearest_face: Result<Face, Error> = faces::table
            .select(Face::as_select())
            .order(faces::embedding.l2_distance(Vector::from(embedding_face.clone())))
            .first(&mut connection());

        let mut person_id: Option<i32> = None;
        if let Ok(face) = nearest_face {
            let similarity =
                cosine_similarity(&face.embedding.unwrap().to_vec(), &embedding_face, true);
            println!(
                "\nSimilarity: {}\nPerson_id: {}\nPhoto_id: {}",
                similarity, face.person_id, face.photo_id
            );
            if similarity > 0.8 {
                person_id = Some(face.person_id);
            }
        }

        if person_id.is_none() {
            person_id = Some(
                diesel::insert_into(persons::table)
                    .values(&NewPerson {
                        title: "Unknown".to_string(),
                        avatar: "storage/avatars/defualt.jpeg".to_string(),
                    })
                    .returning(persons::id)
                    .get_result(&mut connection())
                    .expect("Error saving new person"),
            );
            face.save(format!("storage/avatars/{}.jpeg", person_id.unwrap()))
                .unwrap();

            diesel::update(persons::table)
                .filter(persons::id.eq(person_id.unwrap()))
                .set(persons::avatar.eq(format!("storage/avatars/{}.jpeg", person_id.unwrap())))
                .execute(&mut connection())
                .unwrap();
        }

        let (x_tl, y_tl) = (bbox[0] * img_width as f32, bbox[1] * img_height as f32);
        let (x_br, y_br) = (bbox[2] * img_width as f32, bbox[3] * img_height as f32);

        let res = diesel::insert_into(faces::table)
            .values(&NewFace {
                person_id: person_id.unwrap(),
                photo_id,
                embedding: Some(Vector::from(embedding_face)),
                bbox: Some(
                    [x_tl, y_tl, x_br, y_br]
                        .iter()
                        .map(|e| Some(*e as _))
                        .collect(),
                ),
            })
            .execute(&mut connection());

        match res {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    // println!("{:?}", ultra_output);
}

pub fn cut_image(image: &RgbImage, bbox: &Bbox) -> DynamicImage {
    let (x_tl, y_tl) = (
        bbox[0] * ULTRA_INPUT_WIDTH as f32,
        bbox[1] * ULTRA_INPUT_HEIGHT as f32,
    );
    let (x_br, y_br) = (
        bbox[2] * ULTRA_INPUT_WIDTH as f32,
        bbox[3] * ULTRA_INPUT_HEIGHT as f32,
    );
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

pub async fn upload_image(content: &[u8], img_name: String, upload_dir: String) {
    let img = ImageReader::new(Cursor::new(content))
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
            "models/clip/text/model.onnx",
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
