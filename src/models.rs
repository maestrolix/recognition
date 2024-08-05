use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use diesel::prelude::*;
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
    pub is_admin: bool
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
}

#[derive(Queryable, Selectable, Serialize, Deserialize, ToSchema, Clone, Debug)]
#[diesel(table_name = crate::schema::photos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Photo {
    /// Id изображения
    pub id: i32,
    /// Путь к файлу изображения
    pub path: String,
    /// Наименование изображения
    pub title: Option<String>,
    /// Id пользователя, загрузившего изображение
    pub user_id: i32,
    /// Id альбома
    pub album_id: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize, ToSchema, Clone, Debug, Default)]
#[diesel(table_name = crate::schema::photos)]
pub struct NewPhoto {
    /// Путь к файлу изображения
    pub path: String,
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
            path: "".to_string(),
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
    pub title: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, ToSchema, Clone, Debug, Default)]
#[diesel(table_name = crate::schema::albums)]
pub struct NewAlbum {
    /// Наименование альбома
    pub title: Option<String>,
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
    pub photo_image: FieldData<Bytes>,
    pub title: String,
    pub album_id: i32,
}

#[derive(ToSchema, Debug)]
pub struct FormUtopia {
    pub photo_image: Vec<u8>,
    pub title: String,
    pub user_id: i32,
    pub album_id: i32,
}
