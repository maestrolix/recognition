use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use diesel::prelude::*;
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Queryable, Selectable, Serialize, Deserialize, ToSchema, Clone, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    /// Id пользователя
    pub id: i32,
    /// Имя пользователя
    pub username: String,
    /// Почта пользователя
    pub email: String,
    /// Пароль пользователя
    pub password: String,
    /// Возможности администратора
    pub is_admin: bool,
}

#[derive(Insertable, Serialize, Deserialize, ToSchema, Clone, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    /// Имя пользователя
    pub username: String,
    /// Почта пользователя
    pub email: String,
    /// Пароль пользователя
    pub password: String,
    /// Возможности администратора
    pub is_admin: bool,
}

#[derive(Queryable, Selectable, ToSchema, Clone, Debug)]
#[diesel(table_name = crate::schema::photos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Photo {
    /// Id изображения
    pub id: i32,
    /// Путь к файлу изображения
    pub path: Option<String>,
    /// Наименование изображения
    pub title: Option<String>,
    /// Id пользователя, загрузившего изображение
    pub user_id: i32,
    /// Id альбома
    pub album_id: Option<i32>,
    pub embedding: Option<Vector>,
}

#[derive(Queryable, Serialize, Deserialize, Selectable, ToSchema, Clone, Debug)]
#[diesel(table_name = crate::schema::photos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ListPhoto {
    /// Id изображения
    pub id: i32,
    /// Путь к файлу изображения
    pub path: Option<String>,
    /// Наименование изображения
    pub title: Option<String>,
    /// Id пользователя, загрузившего изображение
    pub user_id: i32,
    /// Id альбома
    pub album_id: Option<i32>,
}

#[derive(Insertable, ToSchema, Clone, Debug, Default)]
#[diesel(table_name = crate::schema::photos)]
pub struct NewPhoto {
    /// Наименование изображения
    pub title: Option<String>,
    /// Id пользователя, загрузившего изображение
    pub user_id: i32,
    /// Id альбома
    pub album_id: Option<i32>,
}

impl NewPhoto {
    pub fn from_form(photo: &PhotoForm, uid: i32) -> Self {
        NewPhoto {
            title: Some(photo.title.clone()),
            user_id: uid,
            album_id: Some(photo.album_id),
        }
    }
}

#[derive(Queryable, Selectable, Serialize, Deserialize, ToSchema, Clone, Debug)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Album {
    /// Id альбома
    pub id: i32,
    /// Наименование альбома
    pub title: String,
}

#[derive(Insertable, Serialize, Deserialize, ToSchema, Clone, Debug, Default)]
#[diesel(table_name = crate::schema::albums)]
pub struct NewAlbum {
    /// Наименование альбома
    pub title: String,
}

#[derive(Deserialize, ToSchema)]
pub struct SignInData {
    /// Почта пользователя
    pub email: String,
    /// Пароль пользователя
    pub password: String,
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct UsersQuery {
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(TryFromMultipart, Debug)]
pub struct PhotoForm {
    pub title: String,
    pub album_id: i32,
    #[form_data(limit = "unlimited")]
    pub photo_image: FieldData<Bytes>,
}

#[derive(ToSchema, Debug)]
pub struct PhotoFormUtopia {
    pub photo_image: Vec<u8>,
    pub title: String,
    pub album_id: i32,
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct SearchQuery {
    pub text: Option<String>,
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct PhotosFilters {
    pub text: Option<String>,
    pub qty: Option<i32>,
}

#[derive(Queryable, Selectable, ToSchema, Clone, Debug)]
#[diesel(table_name = crate::schema::persons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Person {
    /// Id личности
    pub id: i32,
    /// Наименование личности
    pub title: String,
    /// Путь к аватару личности
    pub avatar: String,
}

#[derive(Insertable, ToSchema, Clone, Debug, Default)]
#[diesel(table_name = crate::schema::persons)]
pub struct NewPerson {
    /// Наименование личности
    pub title: String,
    /// Путь к аватару личности
    pub avatar: String,
}

#[derive(Queryable, Selectable, ToSchema, Clone, Debug)]
#[diesel(table_name = crate::schema::faces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Face {
    pub id: i32,
    pub person_id: Option<i32>,
    pub photo_id: i32,
    pub embedding: Option<Vector>,
    pub bbox: Option<Vec<Option<i32>>>,
    pub path: Option<String>,
}

#[derive(Insertable, ToSchema, Clone, Debug, Default)]
#[diesel(table_name = crate::schema::faces)]
pub struct NewFace {
    pub photo_id: i32,
}
