use sqlx::SqlitePool;

use crate::event::logger::Logger;
use crate::{client::StravaClient};

pub struct IngestActivityTask<'a> {
    client: &'a StravaClient,
    pool: &'a SqlitePool,
    logger: Logger,
}

impl IngestActivityTask<'_> {
    pub fn new<'a>(
        client: &'a StravaClient,
        pool: &'a SqlitePool,
        logger: Logger,
    ) -> IngestActivityTask<'a> {
        IngestActivityTask {
            client,
            pool,
            logger,
        }
    }
    pub async fn execute(&mut self) -> Result<(), anyhow::Error> {
        let activity_records = sqlx::query!(
            r#"
            SELECT id FROM raw_activity WHERE activity IS NULL
            "#
        ).fetch_all(self.pool).await?;

        for activity_record in activity_records {
            self.logger.info(format!("Downloading full actiity {}", activity_record.id)).await;

            let s_activity = match self
                .client
                .athlete_activity(format!("{}", activity_record.id))
                .await {
                    Ok(a) => a,
                    Err(err) => {
                        self.logger.info(format!("ERROR activity {}: {}", activity_record.id, err)).await;
                        return Ok(())
                    }
                };

            sqlx::query(
                r#"
                UPDATE raw_activity SET activity = ? WHERE id = ?
                "#,
            ).bind(
                s_activity.to_string(),
            ).bind(
                activity_record.id,
            ).execute(self.pool).await?;
        }
        Ok(())
    }
}
