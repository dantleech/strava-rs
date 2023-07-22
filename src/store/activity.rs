use chrono::NaiveDateTime;
use diesel::prelude::*;
use geo_types::LineString;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::store::schema::activity)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::store::schema::activity_split)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::store::schema::raw_activity)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RawActivity {
    pub id: i64,
    pub listed: String,
    pub activity: Option<String>,
    pub synced: bool,
    pub created_at: NaiveDateTime,
}

pub struct ActivityStore<'a> {
    connection: &'a mut SqliteConnection,
}

impl ActivityStore<'_> {
    pub(crate) fn new(connection: &mut SqliteConnection) -> ActivityStore<'_> {
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

    pub(crate) fn splits(&mut self, activity: Activity) -> Vec<ActivitySplit> {
        use crate::store::schema::activity_split;
        activity_split::table
            .order(activity_split::split.asc())
            .filter(activity_split::activity_id.eq(activity.id))
            .select(ActivitySplit::as_select())
            .load(self.connection)
            .expect("Could not load activity splits")
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
