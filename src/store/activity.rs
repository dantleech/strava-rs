use chrono::NaiveDateTime;
use geo_types::LineString;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Activity {
    pub id: i64,
    pub title: String,
    pub activity_type: String,
    pub description: String,
    pub distance: f64,
    pub average_speed: Option<f64>,
    pub moving_time: i64,
    pub elapsed_time: i64,
    pub total_elevation_gain: f64,
    pub sport_type: String,
    pub average_heartrate: Option<f64>,
    pub max_heartrate: Option<f64>,
    pub start_date: Option<NaiveDateTime>,
    pub summary_polyline: Option<String>,
    pub average_cadence: Option<f64>,
    pub kudos: i64,
    pub location_country: Option<String>,
    pub location_state: Option<String>,
    pub location_city: Option<String>,
    pub athletes: i64,
    pub splits: Vec<ActivitySplit>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct ActivitySplit {
    pub distance: f64,
    pub moving_time: i64,
    pub elapsed_time: i64,
    pub average_speed: f64,
    pub elevation_difference: f64,
    pub split: i64,
}

impl ActivitySplit {
    pub fn seconds_per_meter(&self) -> f64 {
        self.moving_time as f64 / self.distance
    }
}

pub struct ActivityStore<'a> {
    pool: &'a SqlitePool,
}

impl ActivityStore<'_> {
    pub(crate) fn new(pool: &SqlitePool) -> ActivityStore<'_> {
        ActivityStore { pool }
    }

    pub(crate) async fn activities(&mut self) -> Vec<Activity> {
        let activities = sqlx::query!(
            r#"
            SELECT * FROM activity ORDER BY start_date DESC
            "#
        ).fetch_all(self.pool).await.unwrap();

        return activities.iter().map(|rec| {
            let splits: Vec<ActivitySplit> = if let Some(splits) = &rec.activity_splits {
                serde_json::from_str(&splits).unwrap()
            } else {
                vec![]
            };
            Activity{
                id: rec.id,
                title: rec.title.clone(),
                activity_type: rec.activity_type.clone(),
                description: rec.description.clone(),
                distance: rec.distance.clone(),
                average_speed: rec.average_speed.clone(),
                moving_time: rec.moving_time.clone(),
                elapsed_time: rec.elapsed_time.clone(),
                total_elevation_gain: rec.total_elevation_gain.clone(),
                sport_type: rec.sport_type.clone(),
                average_heartrate: rec.average_heartrate.clone(),
                max_heartrate: rec.max_heartrate.clone(),
                start_date: rec.start_date.clone(),
                summary_polyline: rec.summary_polyline.clone(),
                average_cadence: rec.average_cadence.clone(),
                kudos: rec.kudos.clone(),
                location_country: rec.location_country.clone(),
                location_state: rec.location_state.clone(),
                location_city: rec.location_city.clone(),
                athletes: rec.athletes.clone(),
                splits,
            }
        }).collect()
    }
}

impl Activity {
    pub fn time_for_distance(&self, meters: f64) -> i64 {
        ((self.moving_time as f64 / self.distance) * meters) as i64
    }

    pub fn kmph(&self) -> f64 {
        (self.distance / 1000.0) / (self.moving_time as f64 / 3600.0)
    }

    pub fn meters_per_hour(&self) -> f64 {
        self.distance / (self.moving_time as f64 / 3600.0)
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
