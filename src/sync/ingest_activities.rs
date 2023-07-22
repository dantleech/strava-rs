
use std::sync::Arc;

use chrono::{NaiveDateTime};
use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection};
use tokio::sync::mpsc::Sender;

use crate::{
    client::StravaClient,
    store::{activity::RawActivity, schema},
};

pub struct IngestActivitiesTask<'a> {
    client: &'a StravaClient,
    connection: &'a mut SqliteConnection,
    logger: Arc<Sender<String>>,
}

impl IngestActivitiesTask<'_> {
    pub fn new<'a>(
        client: &'a StravaClient,
        connection: &'a mut SqliteConnection,
        logger: Arc<Sender<String>>,
    ) -> IngestActivitiesTask<'a> {
        IngestActivitiesTask { client, connection, logger }
    }
    pub async fn execute(&mut self) -> Result<(), anyhow::Error> {
        use crate::store::schema::raw_activity::dsl::*;
        let mut page: u32 = 0;
        const PAGE_SIZE: u32 = 100;
        let last_epoch = raw_activity
            .select(diesel::dsl::max(created_at))
            .limit(1)
            .first::<Option<NaiveDateTime>>(self.connection)?;

        loop {
            page += 1;
            let s_activities = self
                .client
                .athlete_activities(page, PAGE_SIZE, last_epoch)
                .await?;

            if s_activities.is_empty() {
                self.logger.send("No new activities".to_string()).await?;
                break;
            }

            for s_activity in s_activities {

                self.logger.send(format!("[{}] {}", s_activity["id"], s_activity["name"])).await?;
                let raw = RawActivity {
                    id: s_activity["id"]
                        .as_i64()
                        .expect("could not parse 64 bit ID"),
                    created_at: (match NaiveDateTime::parse_from_str(s_activity["start_date"].as_str().unwrap(), "%Y-%m-%dT%H:%M:%SZ") {
                            Ok(t) => t,
                            Err(_err) => NaiveDateTime::from_timestamp_millis(0).unwrap(),
                        }),
                    listed: s_activity.to_string(),
                    activity: None,
                    synced: false,
                };
                diesel::insert_into(schema::raw_activity::table)
                    .values(&raw)
                    .on_conflict(schema::raw_activity::id)
                    .do_nothing()
                    .execute(self.connection)?;
            }
        }
        Ok(())
    }
}