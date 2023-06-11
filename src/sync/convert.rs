use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::SqliteConnection;
use serde_json::Value;

use crate::client;
use crate::store::activity::Activity;
use crate::store::activity::RawActivity;
use crate::store::schema;

pub struct AcitivityConverter<'a> {
    connection: &'a mut SqliteConnection,
}

impl AcitivityConverter<'_> {
    pub fn new<'a>(connection: &'a mut SqliteConnection) -> AcitivityConverter<'a> {
        AcitivityConverter { connection }
    }
    pub async fn convert(&mut self) -> Result<(), anyhow::Error> {
        use crate::store::schema::raw_activity;

        let raw_activities: Vec<RawActivity> = raw_activity::table
            .select(RawActivity::as_select())
            .filter(raw_activity::synced.eq(false))
            .load(self.connection)?;

        for raw_activity in raw_activities {
            let data: client::Activity =
                serde_json::from_str(&raw_activity.data.as_str()).expect("Could not decode JSON");
            let activity = Activity {
                id: data.id,
                title: data.name,
                activity_type: data.sport_type.clone(),
                distance: data.distance,
                moving_time: data.moving_time,
                elapsed_time: data.elapsed_time,
                total_elevation_gain: data.total_elevation_gain,
                sport_type: data.sport_type.clone(),
                average_heartrate: data.average_heartrate,
                max_heartrate: data.max_heartrate,
                start_date: match data.start_date {
                    Some(date) => Some(date.naive_utc()),
                    None => None,
                },
            };

            diesel::insert_into(schema::activity::table)
                .values(&activity)
                .on_conflict(schema::activity::id)
                .do_nothing()
                .execute(self.connection)?;
        }

        Ok(())
    }
}
