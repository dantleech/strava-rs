use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::store::schema::activity)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Activity {
    pub id: i64,
    pub title: String,
    pub activity_type: String,
    pub distance: f32,
    pub moving_time: i32,
    pub elapsed_time: i32,
    pub total_elevation_gain: f32,
    pub sport_type: String,
    pub average_heartrate: Option<f32>,
    pub max_heartrate: Option<f32>,
    pub start_date: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::store::schema::raw_activity)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RawActivity {
    pub id: i64,
    pub data: String,
    pub synced: bool,
    pub created_at: NaiveDateTime,
}

pub struct ActivityStore<'a> {
    connection: &'a mut SqliteConnection,
}

impl ActivityStore<'_> {
    pub(crate) fn new(connection: &mut SqliteConnection) -> ActivityStore {
        ActivityStore { connection }
    }

    pub(crate) fn activities(&mut self) -> Vec<Activity> {
        use crate::store::schema::activity;
        activity::table
            .order(activity::start_date.desc())
            .select(Activity::as_select())
            .load(self.connection)
            .expect("Could not load activities")
    }
}
