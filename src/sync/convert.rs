use std::collections::HashMap;

use sqlx::SqlitePool;

use crate::client;
use crate::client::Segment;
use crate::event::input::EventSender;
use crate::event::input::InputEvent;
use crate::event::logger::Logger;
use crate::store::activity::Activity;
use crate::store::activity::ActivitySegmentEffort;

pub struct ActivityConverter<'a> {
    pool: &'a SqlitePool,
    event_sender: EventSender,
    logger: Logger,
}

impl ActivityConverter<'_> {
    pub fn new(
        pool: &SqlitePool,
        event_sender: EventSender,
        logger: Logger,
    ) -> ActivityConverter<'_> {
        ActivityConverter {
            pool,
            event_sender,
            logger,
        }
    }
    pub async fn convert(&mut self) -> Result<(), anyhow::Error> {
        let raw_activities = sqlx::query!(
            r#"
            SELECT activity, listed FROM raw_activity WHERE synced = false
            "#
        )
        .fetch_all(self.pool)
        .await?;

        let segments: HashMap<String, Segment> = HashMap::new();
        self.logger.info("Converting activities".to_string()).await;
        let mut i = 0;
        for raw_activity in raw_activities {
            let listed: client::Activity = serde_json::from_str(match &raw_activity.activity {
                Some(a) => a.as_str(),
                None => raw_activity.listed.as_str(),
            })
            .expect("Could not decode JSON");
            if i % 10 == 0 {
                self.logger
                    .info(format!("Converting activity {}", listed.name))
                    .await;
            }
            i += 1;
            let activity = Activity {
                id: listed.id,
                title: listed.name.clone(),
                description: match &listed.description {
                    Some(d) => d.clone(),
                    None => "".to_string(),
                },
                average_speed: listed.average_speed,
                activity_type: listed.sport_type.clone(),
                distance: listed.distance,
                moving_time: listed.moving_time,
                elapsed_time: listed.elapsed_time,
                total_elevation_gain: listed.total_elevation_gain,
                sport_type: listed.sport_type.clone(),
                average_heartrate: listed.average_heartrate,
                max_heartrate: listed.max_heartrate,
                start_date: listed.start_date.map(|date| date.naive_utc()),
                summary_polyline: Some(listed.map.summary_polyline.clone()),
                average_cadence: listed.average_cadence,
                kudos: listed.kudos_count,
                location_country: listed.location_country.clone(),
                location_state: listed.location_state.clone(),
                location_city: listed.location_city.clone(),
                athletes: listed.athlete_count,
                splits: vec![],
                rank: 0,
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
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,?)
                ON CONFLICT DO NOTHING
                "#,
                activity.id,
                activity.title,
                activity.description,
                activity.sport_type,
                activity.distance,
                activity.moving_time,
                activity.elapsed_time,
                activity.total_elevation_gain,
                activity.sport_type,
                activity.average_heartrate,
                activity.max_heartrate,
                activity.start_date,
                activity.summary_polyline,
                activity.average_cadence,
                activity.kudos,
                activity.location_country,
                activity.location_state,
                activity.location_city,
                activity.athletes,
            )
            .execute(self.pool)
            .await?;

            if let Some(full_activity) = raw_activity.activity {
                let activity: client::Activity =
                    serde_json::from_str(full_activity.as_str()).expect("Could not decode JSON");

                let efforts_json = match &activity.segment_efforts {
                    Some(se) => {
                        let a_se: Vec<ActivitySegmentEffort> = se.into_iter().map(|se| ActivitySegmentEffort {
                            segment_id: se.id,
                            elapsed_time: se.elapsed_time,
                            moving_time: se.moving_time,
                            pr_rank: se.pr_rank,
                            kom_rank: se.kom_rank,
                        }).collect();
                        serde_json::to_string(&a_se).unwrap()
                    },
                    None => "[]".to_string(),
                };
                let splits_json = match &activity.splits_standard {
                    Some(s) => {
                        serde_json::to_string(s).unwrap()
                    },
                    None => "[]".to_string(),
                };

                sqlx::query!(
                    r#"
                    UPDATE activity SET activity_splits = ?, segment_efforts = ? WHERE id = ?
                    "#,
                    splits_json,
                    efforts_json,
                    activity.id
                )
                .execute(self.pool)
                .await?;
            }
        }
        self.logger.info("Done converting".to_string()).await;
        self.event_sender.send(InputEvent::Reload).await?;

        Ok(())
    }

    async fn update_segment_sefforts(
        &mut self,
        activity: &client::Activity,
        segment_efforts: &Vec<client::SegmentEffort>,
    ) -> Result<(), anyhow::Error> {
        for segment_effort in segment_efforts {
            let segment = &segment_effort.segment;
            sqlx::query!(
                r#"
                INSERT INTO segment (
                    id,
                    name,
                    distance,
                    average_grade,
                    maximum_grade,
                    elevation_high,
                    elevation_low,
                    start_lat,
                    start_long,
                    end_lat,
                    end_long,
                    climb_category,
                    city,
                    state,
                    country,
                    activity_type,
                    hazardous
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT DO NOTHING
                "#,
                segment.id,
                segment.name,
                segment.distance,
                segment.average_grade,
                segment.maximum_grade,
                segment.elevation_high,
                segment.elevation_low,
                segment.start_latlng.0,
                segment.start_latlng.1,
                segment.end_latlng.0,
                segment.end_latlng.1,
                segment.climb_category,
                segment.city,
                segment.state,
                segment.country,
                activity.sport_type,
                segment.hazardous,
            )
            .execute(self.pool)
            .await?;
            sqlx::query!(
                r#"
                INSERT INTO segment_effort (
                    id,
                    activity_id,
                    moving_time,
                    start_date,
                    average_cadence,
                    device_watts,
                    average_watts,
                    pr_rank,
                    kom_rank
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT DO NOTHING
                "#,
                segment_effort.id,
                activity.id,
                segment_effort.moving_time,
                activity.start_date,
                segment_effort.average_cadence,
                segment_effort.device_watts,
                segment_effort.average_watts,
                segment_effort.pr_rank,
                segment_effort.kom_rank,
            )
            .execute(self.pool)
            .await?;
        }
        Ok(())
    }
}
