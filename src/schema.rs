// @generated automatically by Diesel CLI.

diesel::table! {
    activities (id) {
        id -> Text,
        title -> Text,
        description -> Text,
        status -> Text,
        risk_level -> Text,
        location_lat -> Nullable<Float8>,
        location_lng -> Nullable<Float8>,
        user_id -> Text,
        start_date -> Nullable<Timestamp>,
        end_date -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        is_synchronized -> Bool,
        hashtag -> Nullable<Text>,
        is_deleted -> Nullable<Bool>,
    }
}

diesel::table! {
    activity_photos (id) {
        id -> Text,
        activity_id -> Text,
        url -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password_hash -> Text,
        position -> Text,
        department -> Text,
        phone -> Text,
        status -> Text,
        email_notification -> Nullable<Bool>,
        push_notification -> Nullable<Bool>,
        auto_sync -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    activity_photos,
    users,
);
