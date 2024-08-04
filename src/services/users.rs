use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::models::UsersQuery;
use crate::{auth::hash_password, db_connection::connection, models::*};

pub async fn get_user_by_email(user_email: &str) -> Option<User> {
    use crate::schema::users::dsl::*;

    match users
        .filter(email.eq(user_email))
        .select(User::as_select())
        .first(&mut connection())
    {
        Ok(user) => Some(user),
        _ => None,
    }
}

pub async fn get_users_with_filters(_params: UsersQuery) -> Vec<User> {
    use crate::schema::users::dsl::*;

    users
        // .filter(email.like(params.email))
        // .filter(username.like(params.username))
        .limit(5)
        .select(User::as_select())
        .load(&mut connection())
        .expect("Error loading posts")
}

pub async fn get_user_by_id(user_id: i32) -> User {
    use crate::schema::users::dsl::*;

    users
        .find(user_id)
        .select(User::as_select())
        .first(&mut connection())
        .unwrap()
}

pub async fn delete_user_by_id(user_id: i32) {
    use crate::schema::users::dsl::*;

    diesel::delete(users.filter(id.eq(user_id)))
        .execute(&mut connection())
        .expect("Error deleting posts");
}

pub async fn create_user(mut new_user: NewUser) -> User {
    new_user.password = hash_password(&new_user.password).unwrap();

    diesel::insert_into(crate::schema::users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut connection())
        .expect("Error saving new post")
}
