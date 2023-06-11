// @generated automatically by Diesel CLI.

diesel::table! {
    activity (id) {
        id -> Integer,
        title -> Text,
        activity_type -> Text,
        distance -> Float,
        moving_time -> Integer,
        elapsed_time -> Integer,
        total_elevation_gain -> Float,
        sport_type -> Text,
        average_heartrate -> Nullable<Float>,
        max_heartrate -> Nullable<Float>,
        start_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    raw_activity (id) {
        id -> Integer,
        data -> Text,
        synced -> Bool,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    activity,
    raw_activity,
);
