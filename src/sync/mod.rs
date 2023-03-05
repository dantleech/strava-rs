use crate::{
    client::{self, StravaClient},
    store::activity::{Activity, ActivityStore},
};

pub struct StravaSync<'a> {
    client: &'a StravaClient,
    activity_store: &'a mut ActivityStore,
}

impl StravaSync<'_> {
    pub fn new<'a>(client: &'a StravaClient, activity_store: &'a mut ActivityStore) -> StravaSync {
        StravaSync {
            client,
            activity_store,
        }
    }
    pub async fn sync(&mut self) -> Result<(), anyhow::Error> {
        let mut page: u32 = 0;
        const PAGE_SIZE: u32 = 10;
        let mut offset: u64 = 0;
        self.activity_store.clear();
        loop {
            page+=1;
            let s_activities: Vec<client::Activity> = self.client.athlete_activities(page, PAGE_SIZE).await.unwrap();
            if s_activities.len() == 0 {
                break
            }

            for s_activity in s_activities.iter() {
                offset+=1;
                log::info!("sync: {}: {}", offset, s_activity);
                self.activity_store.add(Activity {
                    name: s_activity.name.clone(),
                    distance: s_activity.distance.clone(),
                    moving_time: s_activity.moving_time.clone(),
                    elapsed_time: s_activity.elapsed_time.clone(),
                    total_elevation_gain: s_activity.total_elevation_gain.clone(),
                    sport_type: s_activity.sport_type.clone(),
                    average_heartrate: s_activity.average_heartrate,
                    max_heartrate: s_activity.max_heartrate,
                    start_date: s_activity.start_date,
                    activity_type: s_activity.sport_type.clone(),
                })
            }

        }
        self.activity_store.flush()?;
        Ok(())
    }
}

