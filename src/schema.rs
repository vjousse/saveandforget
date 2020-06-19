table! {
    documents (id) {
        id -> Int8,
        filename -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    users (email) {
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    documents,
    users,
);
