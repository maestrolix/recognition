use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    db_connection::connection,
    models::{Album, NewAlbum},
};

pub async fn get_album_by_id(album_id: i32) -> Album {
    use crate::schema::albums::dsl::*;

    albums
        .find(album_id)
        .select(Album::as_select())
        .first(&mut connection())
        .unwrap()
}

pub async fn delete_album_by_id(album_id: i32) {
    use crate::schema::albums::dsl::*;

    diesel::delete(albums.filter(id.eq(album_id)))
        .execute(&mut connection())
        .expect("Error deleting posts");
}

pub async fn create_album(new_album: NewAlbum) -> Album {
    diesel::insert_into(crate::schema::albums::table)
        .values(&new_album)
        .returning(Album::as_returning())
        .get_result(&mut connection())
        .expect("Error saving new post")
}
