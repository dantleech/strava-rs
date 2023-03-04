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
        let s_activities: Vec<client::Activity> = self.client.athlete_activities().await.unwrap();

        self.activity_store.clear();
        for s_activity in s_activities.iter() {
            self.activity_store.add(Activity {
                name: s_activity.name.clone(),
                distance: s_activity.distance.clone(),
                moving_time: s_activity.moving_time.clone(),
                elapsed_time: s_activity.elapsed_time.clone(),
                total_elevation_gain: s_activity.total_elevation_gain.clone(),
                sport_type: s_activity.sport_type.clone(),
            })
        }
        self.activity_store.flush()?;
        Ok(())
    }
}

