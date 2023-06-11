use serde_json::Value;

use crate::{
    client::{self, StravaClient},
    store::activity::{Activity, ActivityStore},
};

pub struct StravaSync<'a> {
    client: &'a StravaClient,
    activity_store: &'a mut ActivityStore,
}

impl StravaSync<'_> {
    pub fn new<'a>(client: &'a StravaClient, activity_store: &'a mut ActivityStore) -> StravaSync<'a> {
        StravaSync {
            client,
            activity_store,
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


        }
        Ok(())
    }
}
