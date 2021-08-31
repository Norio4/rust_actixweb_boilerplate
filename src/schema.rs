table! {
    todos (id) {
        id -> Int4,
        text -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        uuid -> Varchar,
        name -> Nullable<Varchar>,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
