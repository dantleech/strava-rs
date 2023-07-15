use chrono::{NaiveDateTime};
use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection};

use crate::{
    client::StravaClient,
    store::{activity::RawActivity, schema},
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
        StravaSync { client, connection }
    }
    pub async fn sync(&mut self) -> Result<(), anyhow::Error> {
        use crate::store::schema::raw_activity::dsl::*;
        let mut page: u32 = 0;
        const PAGE_SIZE: u32 = 10;
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

            if s_activities.len() == 0 {
                break;
            }

            for s_activity in s_activities {

                // strava has a rate limit of 100 requests per 15 minutes, by requesting each
                // individual activity we can easily exceed that.
                //
                // todo: throttle this?
                let s_full_activity = self.client.athlete_activity(s_activity["id"].to_string()).await?;

                let raw = RawActivity {
                    id: s_activity["id"]
                        .as_i64()
                        .expect("could not parse 64 bit ID"),
                    created_at: NaiveDateTime::from(
                        match NaiveDateTime::parse_from_str(s_activity["start_date"].as_str().unwrap(), "%Y-%m-%dT%H:%M:%SZ") {
                            Ok(t) => t,
                            Err(_err) => NaiveDateTime::from_timestamp_millis(0).unwrap(),
                        }
                    ),
                    data: s_full_activity.to_string(),
                    synced: false,
                };
                log::info!("[{}] {}", s_activity["id"], s_activity["name"]);
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
