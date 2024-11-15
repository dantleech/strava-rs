use hyper::Client;
use hyper_rustls::HttpsConnectorBuilder;
use sqlx::SqlitePool;
use tokio::{sync::mpsc::Receiver, task};

use crate::{
    authenticator::Authenticator,
    client::{new_strava_client, StravaConfig},
    event::{input::EventSender, logger::Logger},
};

use self::{
    convert::ActivityConverter, ingest_activities::IngestActivitiesTask,
    ingest_activity::IngestActivityTask,
};

pub mod convert;
pub mod ingest_activities;
pub mod ingest_activity;

pub async fn spawn_sync<'a>(
    pool: SqlitePool,
    event_sender: EventSender,
    client_id: String,
    client_secret: String,
    access_token_path: String,
    logger: Logger,
    mut sync_receiver: Receiver<bool>,
) -> task::JoinHandle<()> {
    let connector = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();
    let event_sender = event_sender;

    task::spawn(async move {
        let client = Client::builder().build(connector);
        let mut authenticator = Authenticator::new(
            client,
            client_id,
            client_secret,
            access_token_path,
            logger.clone(),
        );
        loop {
            let api_config = StravaConfig {
                base_url: "https://www.strava.com/api".to_string(),
                access_token: authenticator.access_token().await.unwrap(),
            };
            let client = new_strava_client(api_config, logger.clone());
            IngestActivitiesTask::new(&client, &pool, logger.clone())
                .execute()
                .await
                .unwrap();
            IngestActivityTask::new(&client, &pool, logger.clone())
                .execute()
                .await
                .unwrap();
            ActivityConverter::new(&pool, event_sender.clone(), logger.clone())
                .convert()
                .await
                .unwrap();
            if sync_receiver.recv().await.is_some() {
                continue;
            }
        }
    })
}
