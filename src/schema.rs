table! {
    files (id) {
        id -> Int4,
        file_name -> Varchar,
        file_type -> Varchar,
        file_data -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        hashed_password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    files,
    users,
);
