use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use super::JsonStorage;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::store::schema::activity)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub id: i32,
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


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::store::schema::raw_activity)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize)]
pub struct RawActivity {
    pub id: i32,
    pub data: String,
    pub synced: bool,
    pub created_at: NaiveDateTime,
}

pub struct ActivityStore {
    activities: Vec<Activity>,
    storage: JsonStorage,
}

impl ActivityStore {
    pub(crate) fn new(storage: JsonStorage) -> ActivityStore {
        ActivityStore {
            activities: storage.load("activities".to_string()),
            storage,
        }
    }

    pub(crate) fn clear(&mut self) -> () {
        self.activities = Vec::new()
    }

    pub(crate) fn add(&mut self, activity: Activity) {
        self.activities.push(activity)
    }

    pub(crate) fn flush(&self) -> Result<(), anyhow::Error> {
        self.storage
            .write("activities".to_string(), &self.activities)?;
        Ok(())
    }

    pub(crate) fn activities(&self) -> &Vec<Activity> {
        &self.activities
    }
}
