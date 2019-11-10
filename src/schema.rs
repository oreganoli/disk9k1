table! {
    content (hash) {
        hash -> Int4,
        mimetype -> Text,
        first_uploaded -> Timestamp,
        times_upload -> Int4,
        times_downloaded -> Int4,
        data -> Bytea,
    }
}

table! {
    directories (id) {
        id -> Int4,
        owner -> Int4,
        name -> Text,
        parent -> Nullable<Int4>,
        created -> Timestamp,
        public -> Bool,
    }
}

table! {
    files (id) {
        id -> Int4,
        filename -> Text,
        hash -> Int4,
        created -> Timestamp,
        updated -> Timestamp,
        directory -> Int4,
        public -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password -> Text,
        quick_token -> Text,
        joined -> Timestamp,
        is_admin -> Bool,
    }
}

joinable!(directories -> users (owner));
joinable!(files -> content (hash));
joinable!(files -> directories (directory));

allow_tables_to_appear_in_same_query!(
    content,
    directories,
    files,
    users,
);
