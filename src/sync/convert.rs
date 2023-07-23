

use diesel::prelude::*;
use diesel::SqliteConnection;


use crate::client;
use crate::store::activity::Activity;
use crate::store::activity::ActivitySplit;
use crate::store::activity::RawActivity;
use crate::store::schema;

use super::logger::LogSender;

pub struct AcitivityConverter<'a> {
    connection: &'a mut SqliteConnection,
    logger: LogSender,
}

impl AcitivityConverter<'_> {
    pub fn new(connection: &mut SqliteConnection, logger: LogSender) -> AcitivityConverter<'_> {
        AcitivityConverter { connection, logger }
    }
    pub async fn convert(&mut self) -> Result<(), anyhow::Error> {
        use crate::store::schema::raw_activity;

        let raw_activities: Vec<RawActivity> = raw_activity::table
            .select(RawActivity::as_select())
            .filter(raw_activity::synced.eq(false))
            .load(self.connection)?;

        for raw_activity in raw_activities {
            let listed: client::Activity =
                serde_json::from_str(raw_activity.listed.as_str()).expect("Could not decode JSON");
            self.logger.info(format!("Converting activity {}", listed.name)).await;
            let activity = Activity {
                id: listed.id,
                title: listed.name,
                description: match listed.description {
                    Some(d) => d,
                    None => "".to_string(),
                },
                activity_type: listed.sport_type.clone(),
                distance: listed.distance,
                moving_time: listed.moving_time,
                elapsed_time: listed.elapsed_time,
                total_elevation_gain: listed.total_elevation_gain,
                sport_type: listed.sport_type.clone(),
                average_heartrate: listed.average_heartrate,
                max_heartrate: listed.max_heartrate,
                start_date: listed.start_date.map(|date| date.naive_utc()),
                summary_polyline: Some(listed.map.summary_polyline),
                average_cadence: listed.average_cadence,
                kudos: listed.kudos_count,
                location_country: listed.location_country,
                location_state: listed.location_state,
                location_city: listed.location_city,
                athletes: listed.athlete_count,
            };

            diesel::insert_into(schema::activity::table)
                .values(&activity)
                .on_conflict(schema::activity::id)
                .do_nothing()
                .execute(self.connection)?;

            if let Some(full_activity) = raw_activity.activity {
                let activity: client::Activity =
                    serde_json::from_str(full_activity.as_str()).expect("Could not decode JSON");
                diesel::delete(schema::activity_split::table.filter(schema::activity_split::activity_id.eq(activity.id))).execute(self.connection)?;

                if let Some(laps) = activity.splits_standard {
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
        }

        Ok(())
    }
}
