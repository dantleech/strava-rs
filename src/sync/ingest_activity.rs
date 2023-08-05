use sqlx::SqliteConnection;

use crate::event::logger::Logger;
use crate::{client::StravaClient};

pub struct IngestActivityTask<'a> {
    client: &'a StravaClient,
    connection: &'a mut SqliteConnection,
    logger: Logger,
}

impl IngestActivityTask<'_> {
    pub fn new<'a>(
        client: &'a StravaClient,
        connection: &'a mut SqliteConnection,
        logger: Logger,
    ) -> IngestActivityTask<'a> {
        IngestActivityTask {
            client,
            connection,
            logger,
        }
    }
    pub async fn execute(&mut self) -> Result<(), anyhow::Error> {
        let activity_records = sqlx::query(
            r#"
            SELECT id FROM raw_activity WHERE activity IS NULL
            "#
        ).fetch_all(self.connection).await;

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

            sqlx::query!(
                r#"
                UPDATE raw_activity SET activity = ? WHERE id = ?
                "#,
                s_activity.to_string(),
                activity_record.id,
            );
        }
        Ok(())
    }
}
