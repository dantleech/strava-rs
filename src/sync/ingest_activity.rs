use std::sync::Arc;

use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection};
use tokio::sync::mpsc::Sender;

use crate::{client::StravaClient, store::activity::RawActivity};

pub struct IngestActivityTask<'a> {
    client: &'a StravaClient,
    connection: &'a mut SqliteConnection,
    logger: Arc<Sender<String>>,
}

impl IngestActivityTask<'_> {
    pub fn new<'a>(
        client: &'a StravaClient,
        connection: &'a mut SqliteConnection,
        logger: Arc<Sender<String>>,
    ) -> IngestActivityTask<'a> {
        IngestActivityTask {
            client,
            connection,
            logger,
        }
    }
    pub async fn execute(&mut self) -> Result<(), anyhow::Error> {
        use crate::store::schema::raw_activity;
        let activities = raw_activity::table
            .select(RawActivity::as_select())
            .filter(raw_activity::activity.is_null())
            .load(self.connection)?;

        for r_activity in activities {
            self.logger
                .send(format!("Downloading full actiity {}", r_activity.id)).await.unwrap();

            let s_activity = match self
                .client
                .athlete_activity(format!("{}", r_activity.id))
                .await {
                    Ok(a) => a,
                    Err(err) => {
                        self.logger
                            .send(format!("ERROR activity {}: {}", r_activity.id, err)).await.unwrap();
                        return Ok(())
                    }
                };

            diesel::update(&r_activity)
                .set(raw_activity::activity.eq(Some(s_activity.to_string())))
                .execute(self.connection)
                .unwrap();
        }
        Ok(())
    }
}
