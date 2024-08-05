// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Nullable<Varchar>,
    }
}

diesel::table! {
    photos (id) {
        id -> Int4,
        path -> Text,
        #[max_length = 50]
        title -> Nullable<Varchar>,
        user_id -> Int4,
        album_id -> Nullable<Int4>,
    }
}

diesel::table! {
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

diesel::joinable!(photos -> albums (album_id));
diesel::joinable!(photos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    photos,
    users,
);
