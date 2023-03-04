pub mod client;
pub mod authenticator;
pub mod store;
pub mod sync;


use std::fs;

use clap::Parser;
use client::{new_strava_client, StravaConfig};
use authenticator::Authenticator;
use hyper::Client;
use hyper_tls::HttpsConnector;
use xdg::BaseDirectories;

use crate::{sync::StravaSync, store::{JsonStorage, activity::ActivityStore}};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    pub client_id: String,
    #[arg(long)]
    pub client_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>{
    env_logger::Builder::new().filter(None, log::LevelFilter::Info).init();

    let connector = HttpsConnector::new();
    let client = Client::builder().build(connector);
    let args = Args::parse();
    let dirs: BaseDirectories = xdg::BaseDirectories::with_prefix("strava-rs").unwrap();
    let access_token_path = dirs.place_state_file("access_token.json").expect("Could not create state directory");
    let storage_path = dirs.get_data_home();

    let mut authenticator = Authenticator::new(
        client,
        args.client_id,
        args.client_secret,
        access_token_path.to_str().unwrap().to_string()
    );

    let api_config = StravaConfig {
        base_url: "https://www.strava.com/api".to_string(),
        access_token: authenticator.access_token().await?,
    };
    let json_storage = JsonStorage::new(storage_path.to_str().unwrap().to_string());
    let mut activity_store = ActivityStore::new(json_storage);
    let client = new_strava_client(api_config);

    log::info!("Strava TUI");
    log::info!("==========");
    log::info!("");
    log::info!("Token path: {}", access_token_path.display());
    log::info!("Storage path: {}", storage_path.display());
    log::info!("");

    StravaSync::new(&client,&mut activity_store).sync().await?;

    println!("{:#?} Activites", client.athlete_activities().await?.len());
    Ok(())
}
