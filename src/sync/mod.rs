
use diesel::{r2d2::{ConnectionManager, Pool}, SqliteConnection};
use hyper::Client;
use hyper_rustls::HttpsConnectorBuilder;
use sqlx::SqlitePool;
use tokio::{task, sync::mpsc::Receiver};

use crate::{authenticator::Authenticator, event::{input::EventSender, logger::Logger}, client::{StravaConfig, new_strava_client}};

use self::{ingest_activities::IngestActivitiesTask, ingest_activity::IngestActivityTask, convert::AcitivityConverter};

pub mod convert;
pub mod ingest_activities;
pub mod ingest_activity;

pub async fn spawn_sync(
    pool: &SqlitePool,
    event_sender: EventSender,
    client_id: String,
    client_secret: String,
    access_token_path: String,
    logger: Logger,
    mut sync_receiver: Receiver<bool>,
) -> task::JoinHandle<()> {
    let connector = HttpsConnectorBuilder::new().with_native_roots().https_only().enable_http1().build();
    let mut sync_conn = pool.acquire().await.unwrap();
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
                IngestActivitiesTask::new(&client, &mut sync_conn, logger.clone())
                    .execute()
                    .await
                    .unwrap();
                IngestActivityTask::new(&client, &mut sync_conn, logger.clone())
                    .execute()
                    .await
                    .unwrap();
                AcitivityConverter::new(&mut sync_conn, event_sender.clone(), logger.clone())
                    .convert()
                    .await
                    .unwrap();
            if sync_receiver.recv().await.is_some() {
                continue;
            }
        }
    })
}
