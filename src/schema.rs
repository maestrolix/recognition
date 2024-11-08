// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    albums (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    faces (id) {
        id -> Int4,
        person_id -> Nullable<Int4>,
        photo_id -> Int4,
        embedding -> Nullable<Vector>,
        path -> Nullable<Text>,
        bbox -> Nullable<Array<Nullable<Int4>>>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    persons (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        avatar -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    photos (id) {
        id -> Int4,
        path -> Nullable<Text>,
        #[max_length = 50]
        title -> Nullable<Varchar>,
        embedding -> Nullable<Vector>,
        user_id -> Int4,
        album_id -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 50]
        email -> Varchar,
        #[max_length = 256]
        password -> Varchar,
        avatar -> Nullable<Text>,
        is_admin -> Bool,
    }
}

diesel::joinable!(faces -> persons (person_id));
diesel::joinable!(faces -> photos (photo_id));
diesel::joinable!(photos -> albums (album_id));
diesel::joinable!(photos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    faces,
    persons,
    photos,
    users,
);
