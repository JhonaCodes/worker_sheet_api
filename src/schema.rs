// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    workers (id) {
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(users, workers,);
