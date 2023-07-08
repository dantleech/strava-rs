// @generated automatically by Diesel CLI.

diesel::table! {
    activity (id) {
        id -> BigInt,
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
        summary_polyline -> Nullable<Text>,
        average_cadence -> Nullable<Float>,
        average_speed -> Nullable<Float>,
        kudos -> Integer,
        location_country -> Nullable<Text>,
        location_state -> Nullable<Text>,
        location_city -> Nullable<Text>,
        athletes -> Integer,
    }
}

diesel::table! {
    activity_lap (id) {
        id -> BigInt,
        activity_id -> Integer,
        name -> Text,
        distance -> Float,
        moving_time -> Integer,
        elapsed_time -> Integer,
        total_elevation_gain -> Float,
        average_cadence -> Nullable<Float>,
    }
}

diesel::table! {
    raw_activity (id) {
        id -> BigInt,
        data -> Text,
        synced -> Bool,
        created_at -> Timestamp,
    }
}

diesel::joinable!(activity_lap -> activity (activity_id));

diesel::allow_tables_to_appear_in_same_query!(
    activity,
    activity_lap,
    raw_activity,
);
