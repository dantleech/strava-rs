use chrono::{Local, NaiveDate, NaiveDateTime};
use diesel::{expression::AsExpression, RunQueryDsl, SqliteConnection};
use serde_json::Value;

use crate::{
    client::{self, StravaClient},
    store::{
        activity::{Activity, ActivityStore, RawActivity},
        schema,
    },
};

pub struct StravaSync<'a> {
    client: &'a StravaClient,
    connection: &'a mut SqliteConnection,
}

impl StravaSync<'_> {
    pub fn new<'a>(
        client: &'a StravaClient,
        connection: &'a mut SqliteConnection,
    ) -> StravaSync<'a> {
        StravaSync {
            client,
            connection,
        }
    }
    pub async fn sync(&mut self) -> Result<(), anyhow::Error> {
        let mut page: u32 = 0;
        const PAGE_SIZE: u32 = 10;
        let mut offset: u64 = 0;

        loop {
            page += 1;
            let s_activities = self
                .client
                .athlete_activities(page, PAGE_SIZE)
                .await
                .unwrap();

            if s_activities.len() == 0 {
                break;
            }

            for s_activity in s_activities {
                offset += 1;
                let raw = RawActivity {
                    id: s_activity["id"]
                        .as_i64()
                        .expect("could not parse 64 bit ID"),
                    created_at: Local::now().naive_local(),
                    data: s_activity.to_string(),
                    synced: false,
                };
                diesel::insert_into(schema::raw_activity::table)
                    .values(&raw)
                    .execute(self.connection)?;
            }
        }
        Ok(())
    }
}
