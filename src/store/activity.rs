use std::{cmp::Ordering, collections::HashMap, fmt::Display};

use chrono::NaiveDateTime;
use crossterm::event::KeyCode;
use geo_types::LineString;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use strum::EnumIter;

use crate::expr::{
    evaluator::{Evaluator, Evalue, Vars},
    parser::Expr,
};

use super::polyline_compare::compare;

#[derive(EnumIter)]
pub enum SortBy {
    Date,
    Distance,
    Pace,
    HeartRate,
    Time,
}
pub enum SportType {
    Ride,
    Run,
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
                SortOrder::Asc => "asc",
                SortOrder::Desc => "desc",
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

impl Default for Activities {
    fn default() -> Self {
        Self::new()
    }
}

impl Activities {
    pub fn new() -> Activities {
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
        self.activities.iter().find(|a| a.id == id)
    }

    pub fn where_title_contains(&self, pattern: &str) -> Activities {
        self.activities
            .clone()
            .into_iter()
            .filter(|a| a.title.contains(pattern))
            .collect()
    }

    pub fn having_activity_type(&self, activity_type: String) -> Activities {
        self.activities
            .clone()
            .into_iter()
            .filter(|a| a.activity_type == activity_type)
            .collect()
    }

    pub fn withing_distance_of(&self, anchored: &Activity, tolerant: f64) -> Activities {
        self.activities
            .clone()
            .into_iter()
            .filter(|a| {
                if anchored.polyline().is_err() || a.polyline().is_err() {
                    return false;
                }
                compare(&anchored.polyline().unwrap(), &a.polyline().unwrap(), 100) < tolerant
            })
            .collect()
    }

    pub fn sort(&self, sort_by: &SortBy, sort_order: &SortOrder) -> Activities {
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

    pub fn rank(&self, rank_by: &SortBy, rank_order: &SortOrder) -> Activities {
        let sorted = self.sort(rank_by, rank_order);
        let s = sorted.to_vec();
        let mut rank = 0;

        let s = s.iter().cloned().map(|a| {
            let mut aa = a;
            rank += 1;
            aa.rank = rank;
            aa
        });
        Activities::from(s.collect::<Vec<Activity>>())
    }

    pub fn is_empty(&self) -> bool {
        self.activities.is_empty()
    }

    pub fn timestamps(&self) -> Vec<i64> {
        self.activities
            .iter()
            .map(|a| a.start_date.unwrap().and_utc().timestamp())
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

    pub(crate) fn by_expr(&self, evaluator: &Evaluator, expr: &Expr) -> Activities {
        self.activities
            .clone()
            .into_iter()
            .filter(|a| {
                evaluator
                    .evaluate(
                        expr,
                        &Vars::from([
                            ("distance".to_string(), Evalue::Number(a.distance)),
                            (
                                "type".to_string(),
                                Evalue::String(a.activity_type.to_string()),
                            ),
                            (
                                "heartrate".to_string(),
                                Evalue::Number(a.average_heartrate.unwrap_or(0.0)),
                            ),
                            ("title".to_string(), Evalue::String(a.title.clone())),
                            (
                                "elevation".to_string(),
                                Evalue::Number(a.total_elevation_gain),
                            ),
                            ("time".to_string(), Evalue::Number(a.moving_time as f64)),
                            (
                                "date".to_string(),
                                Evalue::Date(a.start_date.unwrap_or_default().into()),
                            ),
                            ("speed".to_string(), Evalue::Number(a.meters_per_hour())),
                        ]),
                    )
                    .unwrap_or_default()
            })
            .collect()
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
    pub segment_efforts: Vec<ActivitySegmentEffort>,
    pub rank: i64,
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

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct ActivitySegmentEffort {
    pub segment_id: i64,
    pub elapsed_time: i64,
    pub moving_time: i64,
    pub pr_rank: Option<u8>,
    pub kom_rank: Option<u8>,
}
impl ActivitySegmentEffort {
    pub fn meters_per_hour(&self, distance: f64) -> f64 {
        distance / (self.moving_time as f64 / 3600.0)
    }
}

impl ActivitySplit {
    pub fn seconds_per_meter(&self) -> f64 {
        self.moving_time as f64 / self.distance
    }

    pub(crate) fn meters_per_hour(&self) -> f64 {
        self.distance / (self.moving_time as f64 / 3600.0)
    }
}

pub struct ActivityStore<'a> {
    pool: &'a SqlitePool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Segment {
    pub id: i64,
    pub name: String,
    pub distance: f64,
    pub activity_type: String,
}

impl ActivityStore<'_> {
    pub fn new(pool: &SqlitePool) -> ActivityStore<'_> {
        ActivityStore { pool }
    }
    pub async fn segments(&mut self) -> HashMap<i64, Segment> {
        let segments = sqlx::query!("SELECT id, name, distance, activity_type FROM segment")
            .fetch_all(self.pool)
            .await
            .unwrap();
        segments
            .iter()
            .map(|rec| {
                (
                    rec.id,
                    Segment {
                        id: rec.id,
                        name: rec.name.clone(),
                        distance: rec.distance,
                        activity_type: rec.activity_type.clone(),
                    },
                )
            })
            .collect()
    }

    pub async fn activities(&mut self) -> Activities {
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
                let efforts: Vec<ActivitySegmentEffort> =
                    if let Some(efforts) = &rec.segment_efforts {
                        serde_json::from_str(efforts).unwrap()
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
                    segment_efforts: efforts,
                    athletes: rec.athletes,
                    splits,
                    rank: 0,
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

    pub fn activity_type_icon(&self) -> String {
        match self.activity_type.as_str() {
            "Ride" => "🚴".to_string(),
            "Run" => "🏃".to_string(),
            "TrailRun" => "🏃".to_string(),
            "Walk" => "🥾".to_string(),
            _ => "❓".to_string(),
        }
    }

    pub fn polyline(&self) -> Result<Polyline, String> {
        if let Some(p) = &self.summary_polyline {
            return polyline::decode_polyline(p.as_str(), 5);
        }

        Err("No polyline".to_string())
    }
}
