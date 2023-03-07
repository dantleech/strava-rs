use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::JsonStorage;

#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub name: String,
    pub activity_type: String,
    pub distance: f32,
    pub moving_time: u32,
    pub elapsed_time: u32,
    pub total_elevation_gain: f32,
    pub sport_type: String,
    pub average_heartrate: Option<f32>,
    pub max_heartrate: Option<f32>,
    pub start_date: DateTime<Utc>,
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
