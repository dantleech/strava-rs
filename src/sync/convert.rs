use std::sync::Arc;

use diesel::prelude::*;
use diesel::SqliteConnection;
use tokio::sync::mpsc::Sender;

use crate::client;
use crate::store::activity::Activity;
use crate::store::activity::ActivitySplit;
use crate::store::activity::RawActivity;
use crate::store::schema;

pub struct AcitivityConverter<'a> {
    connection: &'a mut SqliteConnection,
}

impl AcitivityConverter<'_> {
    pub fn new(connection: &mut SqliteConnection, _sender: Arc<Sender<String>>) -> AcitivityConverter<'_> {
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
                serde_json::from_str(raw_activity.listed.as_str()).expect("Could not decode JSON");
            let activity = Activity {
                id: data.id,
                title: data.name,
                description: match data.description {
                    Some(d) => d,
                    None => "".to_string(),
                },
                activity_type: data.sport_type.clone(),
                distance: data.distance,
                moving_time: data.moving_time,
                elapsed_time: data.elapsed_time,
                total_elevation_gain: data.total_elevation_gain,
                sport_type: data.sport_type.clone(),
                average_heartrate: data.average_heartrate,
                max_heartrate: data.max_heartrate,
                start_date: data.start_date.map(|date| date.naive_utc()),
                summary_polyline: Some(data.map.summary_polyline),
                average_cadence: data.average_cadence,
                kudos: data.kudos_count,
                location_country: data.location_country,
                location_state: data.location_state,
                location_city: data.location_city,
                athletes: data.athlete_count,
            };

            diesel::insert_into(schema::activity::table)
                .values(&activity)
                .on_conflict(schema::activity::id)
                .do_nothing()
                .execute(self.connection)?;
            diesel::delete(schema::activity_split::table.filter(schema::activity_split::activity_id.eq(activity.id))).execute(self.connection)?;

            if let Some(laps) = data.splits_standard {
                let mut activity_laps: Vec<ActivitySplit> = vec![];
                for lap in laps {
                    activity_laps.push(ActivitySplit{
                        activity_id: activity.id,
                        distance: lap.distance,
                        moving_time: lap.moving_time,
                        elapsed_time: lap.elapsed_time,
                        average_speed: lap.average_speed,
                        elevation_difference: lap.elevation_difference,
                        split: lap.split,
                    });
                }
                    
                diesel::insert_into(schema::activity_split::table)
                    .values(&activity_laps)
                    .execute(self.connection)?;
            }
        }

        Ok(())
    }
}
