use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::JsonStorage;


#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub name: String,
    pub distance: f64,
    pub moving_time: u64,
    pub elapsed_time: u64,
    pub total_elevation_gain: f64,
    pub sport_type: String,
    pub average_heartrate: Option<f64>,
    pub max_heartrate: Option<f64>,
    pub start_date: DateTime<Utc>,
}

pub struct ActivityStore {
    activities: Vec<Activity>,
    storage: JsonStorage,
}

impl ActivityStore {

    pub(crate) fn clear(&mut self) -> () {
        self.activities = Vec::new()
    }

    pub(crate) fn add(&mut self, activity: Activity) {
        self.activities.push(activity)
    }

    pub(crate) fn new(storage: JsonStorage) -> ActivityStore {
        ActivityStore { activities: Vec::new(), storage }
    }

    pub(crate) fn flush(&self) -> Result<(), anyhow::Error> {
        self.storage.write("activities".to_string(), &self.activities)?;
        Ok(())
    }
}
