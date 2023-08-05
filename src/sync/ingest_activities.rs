use chrono::{NaiveDateTime};
use sqlx::SqliteConnection;

use crate::event::logger::Logger;
use crate::{
    client::StravaClient,
};


pub struct IngestActivitiesTask<'a> {
    client: &'a StravaClient,
    connection: &'a mut SqliteConnection,
    logger: Logger,
}

impl IngestActivitiesTask<'_> {
    pub fn new<'a>(
        client: &'a StravaClient,
        connection: &'a mut SqliteConnection,
        logger: Logger,
    ) -> IngestActivitiesTask<'a> {
        IngestActivitiesTask { client, connection, logger }
    }
    pub async fn execute(&mut self) -> Result<(), anyhow::Error> {
        let mut page: u32 = 0;
        const PAGE_SIZE: u32 = 100;
        let last = sqlx::query!(
            r#"
SELECT MAX(created_at) as epoch FROM raw_activity LIMIT 1
            "#
        ).fetch_one(&mut self.connection).await?;

        loop {
            page += 1;
            let s_activities = match self
                .client
                .athlete_activities(page, PAGE_SIZE, last.epoch)
                .await {
                    Ok(a) => a,
                    Err(e) => {
                        self.logger.error(format!("Error: {}", e)).await;
                        return Ok(())
                    },
                };

            if s_activities.is_empty() {
                self.logger.info("Non new activities".to_string()).await;
                break;
            }

            for s_activity in s_activities {
                self.logger.info(format!("[{}] {}", s_activity["id"], s_activity["name"])).await;
                sqlx::query!(
                    r#"
INSERT INTO raw_activity (id, created_at, listed, synced) VALUES (?, ?, ?, false) ON CONFLICT(id) DO NOTHING
                    "#,
                    s_activity["id"]
                        .as_i64()
                        .expect("could not parse 64 bit ID"),
                    (match NaiveDateTime::parse_from_str(s_activity["start_date"].as_str().unwrap(), "%Y-%m-%dT%H:%M:%SZ") {
                            Ok(t) => t,
                            Err(_err) => NaiveDateTime::from_timestamp_millis(0).unwrap(),
                    }),
                    s_activity.to_string()
                );
            }
        }
        Ok(())
    }
}
