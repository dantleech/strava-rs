
use libsqlite3_sys::sqlite3_expired;
use sqlx::QueryBuilder;
use sqlx::SqliteConnection;

use crate::client;
use crate::event::input::EventSender;
use crate::event::input::InputEvent;
use crate::event::logger::Logger;
use crate::store::activity::Activity;
use crate::store::activity::ActivitySplit;

pub struct AcitivityConverter<'a> {
    connection: &'a mut SqliteConnection,
    event_sender: EventSender,
    logger: Logger,
}

impl AcitivityConverter<'_> {
    pub fn new(
        connection: &mut SqliteConnection,
        event_sender: EventSender,
        logger: Logger,
    ) -> AcitivityConverter<'_> {
        AcitivityConverter {
            connection,
            event_sender,
            logger,
        }
    }
    pub async fn convert(&mut self) -> Result<(), anyhow::Error> {
        let raw_activities = sqlx::query!(
            r#"
            SELECT activity, listed FROM raw_activity WHERE synced = false
            "#
        ).fetch_all(self.connection).await?;

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

            sqlx::query!(
                r#"
                INSERT INTO activity (
                    id,
                    title,
                    description,
                    activity_type,
                    distance,
                    moving_time,
                    elapsed_time,
                    total_elevation_gain,
                    sport_type,
                    average_heartrate,
                    max_heartrate,
                    start_date,
                    summary_polyline,
                    average_cadence,
                    kudos,
                    location_country,
                    location_state,
                    location_city,
                    athletes
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT DO NOTHING
                "#,
                listed.id,
                listed.name,
                match listed.description {
                    Some(d) => d,
                    None => "".to_string(),
                },
                listed.sport_type.clone(),
                listed.distance,
                listed.moving_time,
                listed.elapsed_time,
                listed.total_elevation_gain,
                listed.sport_type.clone(),
                listed.average_heartrate,
                listed.max_heartrate,
                listed.start_date.map(|date| date.naive_utc()),
                Some(listed.map.summary_polyline),
                listed.average_cadence,
                listed.kudos_count,
                listed.location_country,
                listed.location_state,
                listed.location_city,
                listed.athlete_count,
            ).execute(self.connection).await;

            if let Some(full_activity) = raw_activity.activity {
                let activity: client::Activity =
                    serde_json::from_str(full_activity.as_str()).expect("Could not decode JSON");
                sqlx::query!(
                    r#"
                    DELETE FROM activity_split WHERE activity_id = ?
                    "#,
                    activity.id
                ).execute(self.connection).await?;

                if let Some(laps) = activity.splits_standard {
                    let mut activity_laps: Vec<ActivitySplit> = vec![];
                    for lap in laps {
                        activity_laps.push(ActivitySplit {
                            activity_id: activity.id,
                            distance: lap.distance,
                            moving_time: lap.moving_time,
                            elapsed_time: lap.elapsed_time,
                            average_speed: lap.average_speed,
                            elevation_difference: lap.elevation_difference,
                            split: lap.split,
                        });
                    }
                    let qb = QueryBuilder::new(
                        r#"
                        INSERT INTO activity_lap (activity_id, distance, moving_time, elapsed-time, average_speed, elevation_difference, split)")
                        "#
                    );
                    qb.push_values(activity_laps, |mut b, activity_lap| {
                        b.push_bind(activity_lap.id);
                        b.push_bind(activity_lap.distance);
                        b.push_bind(activity_lap.moving_time);
                        b.push_bind(activity_lap.elapsed_time);
                        b.push_bind(activity_lap.average_speed);
                        b.push_bind(activity_lap.elevation_difference);
                        b.push_bind(activity_lap.split);
                    });
                    qb.build().execute(self.connection).await?;
                }
            }
        }
        self.logger.info("Done converting".to_string()).await;
        self.event_sender.send(InputEvent::Reload).await?;

        Ok(())
    }
}
