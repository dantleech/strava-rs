use chrono::NaiveDateTime;
use geo_types::LineString;
use serde::{Deserialize, Serialize};
use sqlx::{SqliteConnection, FromRow};

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Activity {
    pub id: i64,
    pub title: String,
    pub activity_type: String,
    pub description: String,
    pub distance: f32,
    pub moving_time: i32,
    pub elapsed_time: i32,
    pub total_elevation_gain: f32,
    pub sport_type: String,
    pub average_heartrate: Option<f32>,
    pub max_heartrate: Option<f32>,
    pub start_date: Option<NaiveDateTime>,
    pub summary_polyline: Option<String>,
    pub average_cadence: Option<f32>,
    pub kudos: i32,
    pub location_country: Option<String>,
    pub location_state: Option<String>,
    pub location_city: Option<String>,
    pub athletes: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct ActivitySplit {
    pub activity_id: i64,
    pub distance: f32,
    pub moving_time: i32,
    pub elapsed_time: i32,
    pub average_speed: f32,
    pub elevation_difference: f32,
    pub split: i32,
}

impl ActivitySplit {
    pub fn seconds_per_meter(&self) -> f32 {
        self.moving_time as f32 / self.distance
    }
}

pub struct ActivityStore<'a> {
    connection: &'a mut SqliteConnection,
}

impl ActivityStore<'_> {
    pub(crate) fn new(connection: &mut SqliteConnection) -> ActivityStore<'_> {
        ActivityStore { connection }
    }

    pub(crate) async fn activities(&mut self) -> Vec<Activity> {
        sqlx::query_as::<_, Activity>(
            r#"
            SELECT * FROM activity ORDER BY start_date DESC
            "#
        ).fetch_all(self.connection).await.unwrap()
    }

    pub(crate) async fn splits(&mut self, activity: Activity) -> Vec<ActivitySplit> {
        sqlx::query_as::<_, ActivitySplit>(
            r#"
            SELECT * FROM activity_split WHERE activity_id = ? ORDER BY split ASC
            "#
        ).fetch_all(self.connection).await.expect("Could not load activity splits")
    }
}

impl Activity {
    pub fn time_for_distance(&self, meters: f32) -> i32 {
        ((self.moving_time as f32 / self.distance) * meters) as i32
    }

    pub fn kmph(&self) -> f32 {
        (self.distance / 1000.0) / (self.moving_time as f32 / 3600.0)
    }

    pub fn meters_per_hour(&self) -> f32 {
        self.distance / (self.moving_time as f32 / 3600.0)
    }

    pub(crate) fn activity_type_icon(&self) -> String {
        match self.activity_type.as_str() {
            "Ride" => "🚴".to_string(),
            "Run" => "🏃".to_string(),
            "TrailRun" => "🏃".to_string(),
            "Walk" => "🥾".to_string(),
            _ => "❓".to_string(),
        }
    }

    pub(crate) fn polyline(&self) -> Result<LineString, String> {
        if let Some(p) = &self.summary_polyline {
            return polyline::decode_polyline(p.as_str(), 5);
        }

        Err("No polyline".to_string())
    }
}
