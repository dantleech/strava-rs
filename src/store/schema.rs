// @generated automatically by Diesel CLI.

diesel::table! {
    activity (id) {
        id -> Nullable<Integer>,
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
