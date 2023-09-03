use std::{cmp::Ordering, fmt::Display};

use chrono::NaiveDateTime;
use crossterm::event::KeyCode;
use geo_types::LineString;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use strum::EnumIter;

use super::polyline_compare::compare;

#[derive(EnumIter)]
pub enum SortBy {
    Date,
    Distance,
    Pace,
    HeartRate,
    Time,
}

impl Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_label())
    }
}

impl SortBy {
    pub fn to_key(&self) -> char {
        match *self {
            SortBy::Date => 'd',
            SortBy::Pace => 'p',
            SortBy::HeartRate => 'h',
            SortBy::Distance => 'D',
            SortBy::Time => 't',
        }
    }

    pub fn to_label(&self) -> &str {
        match *self {
            SortBy::Date => "date",
            SortBy::Pace => "pace",
            SortBy::HeartRate => "heartrate",
            SortBy::Distance => "distance",
            SortBy::Time => "time",
        }
    }

    pub fn from_key(key: KeyCode) -> Option<SortBy> {
        match key {
            KeyCode::Char('d') => Some(SortBy::Date),
            KeyCode::Char('p') => Some(SortBy::Pace),
            KeyCode::Char('h') => Some(SortBy::HeartRate),
            KeyCode::Char('D') => Some(SortBy::Distance),
            KeyCode::Char('t') => Some(SortBy::Time),
            _ => None,
        }
    }
}

pub enum SortOrder {
    Asc,
    Desc,
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SortOrder::Asc => "ascending",
                SortOrder::Desc => "descending",
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct Activities {
    activities: Vec<Activity>,
    offset: usize,
}

impl Iterator for Activities {
    type Item = Activity;

    fn next(&mut self) -> Option<Self::Item> {
        match self.activities.get(self.offset + 1) {
            Some(a) => {
                self.offset += 1;
                Some(a.clone())
            }
            None => None,
        }
    }
}

impl Activities {
    pub(crate) fn new() -> Activities {
        Self {
            activities: vec![],
            offset: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.activities.len()
    }
    pub fn get(&self, offset: usize) -> Option<&Activity> {
        self.activities.get(offset)
    }

    pub fn find(&self, id: i64) -> Option<&Activity> {
        self.activities.iter().find(|a|a.id == id)
    }

    pub(crate) fn where_title_contains(&self, pattern: &str) -> Activities {
        self.activities.clone()
            .into_iter()
            .filter(|a| a.title.contains(pattern))
            .collect()
    }

    pub(crate) fn having_activity_type(&self, activity_type: String) -> Activities {
        self.activities.clone()
            .into_iter()
            .filter(|a| a.activity_type == activity_type)
            .collect()
    }

    pub(crate) fn withing_distance_of(&self, anchored: &Activity, tolerant: f64) -> Activities {
        self.activities.clone()
            .into_iter()
            .filter(|a| {
                if anchored.polyline().is_err() || a.polyline().is_err() {
                    return false;
                }
                compare(&anchored.polyline().unwrap(), &a.polyline().unwrap(), 100) < tolerant
            })
            .collect()
    }

    pub(crate) fn sort(
        &self,
        sort_by: &SortBy,
        sort_order: &SortOrder,
    ) -> Activities {
        let mut activities = self.activities.clone();
        activities.sort_by(|a, b| {
            let ordering = match sort_by {
                SortBy::Date => a.id.cmp(&b.id),
                SortBy::Distance => a
                    .distance
                    .partial_cmp(&b.distance)
                    .unwrap_or(Ordering::Less),
                SortBy::Pace => a.kmph().partial_cmp(&b.kmph()).unwrap_or(Ordering::Less),
                SortBy::HeartRate => a
                    .average_heartrate
                    .or(Some(0.0))
                    .partial_cmp(&b.average_heartrate.or(Some(0.0)))
                    .unwrap(),
                SortBy::Time => a.moving_time.partial_cmp(&b.moving_time).unwrap(),
            };
            match sort_order {
                SortOrder::Asc => ordering,
                SortOrder::Desc => ordering.reverse(),
            }
        });
        Activities::from(activities)
    }


    pub fn is_empty(&self) -> bool {
        self.activities.is_empty()
    }

    pub fn timestamps(&self) -> Vec<i64> {
        self.activities
            .iter()
            .map(|a| a.start_date.unwrap().timestamp())
            .collect()
    }

    pub fn meter_per_hours(&self) -> Vec<i64> {
        self.activities
            .iter()
            .map(|a| a.meters_per_hour() as i64)
            .collect()
    }

    pub fn to_vec(&self) -> Vec<Activity> {
        self.activities.clone()
    }
}

impl From<Vec<Activity>> for Activities {
    fn from(activities: Vec<Activity>) -> Self {
        Activities {
            activities,
            offset: 0,
        }
    }
}
impl From<Activity> for Activities {
    fn from(activity: Activity) -> Self {
        Activities {
            activities: vec![activity],
            offset: 0,
        }
    }
}

impl FromIterator<Activity> for Activities {
    fn from_iter<T: IntoIterator<Item = Activity>>(iter: T) -> Self {
        Activities {
            activities: Vec::from_iter(iter),
            offset: 0,
        }
    }
}

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

    pub(crate) async fn activities(&mut self) -> Activities {
        let activities = sqlx::query!(
            r#"
            SELECT * FROM activity ORDER BY start_date DESC
            "#
        )
        .fetch_all(self.pool)
        .await
        .unwrap();

        return activities
            .iter()
            .map(|rec| {
                let splits: Vec<ActivitySplit> = if let Some(splits) = &rec.activity_splits {
                    serde_json::from_str(splits).unwrap()
                } else {
                    vec![]
                };
                Activity {
                    id: rec.id,
                    title: rec.title.clone(),
                    activity_type: rec.activity_type.clone(),
                    description: rec.description.clone(),
                    distance: rec.distance,
                    average_speed: rec.average_speed,
                    moving_time: rec.moving_time,
                    elapsed_time: rec.elapsed_time,
                    total_elevation_gain: rec.total_elevation_gain,
                    sport_type: rec.sport_type.clone(),
                    average_heartrate: rec.average_heartrate,
                    max_heartrate: rec.max_heartrate,
                    start_date: rec.start_date,
                    summary_polyline: rec.summary_polyline.clone(),
                    average_cadence: rec.average_cadence,
                    kudos: rec.kudos,
                    location_country: rec.location_country.clone(),
                    location_state: rec.location_state.clone(),
                    location_city: rec.location_city.clone(),
                    athletes: rec.athletes,
                    splits,
                }
            })
            .collect();
    }
}

pub type Polyline = LineString;

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

    pub(crate) fn polyline(&self) -> Result<Polyline, String> {
        if let Some(p) = &self.summary_polyline {
            return polyline::decode_polyline(p.as_str(), 5);
        }

        Err("No polyline".to_string())
    }
}
