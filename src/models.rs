use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Selectable, Serialize, Deserialize, ToSchema, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}


#[derive(Insertable, Serialize, Deserialize, ToSchema, Clone, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}