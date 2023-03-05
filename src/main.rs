pub mod authenticator;
pub mod client;
pub mod store;
pub mod sync;
pub mod ui;

use std::{fs, io};

use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use authenticator::Authenticator;
use clap::Parser;
use client::{new_strava_client, StravaConfig};
use hyper::Client;
use hyper_tls::HttpsConnector;
use tui::{Terminal, backend::CrosstermBackend};
use xdg::BaseDirectories;

use crate::{
    store::{activity::ActivityStore, JsonStorage},
    sync::StravaSync, ui::{layout::AppLayout, app::App, activity_list::ActivityList},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short,long)]
    pub no_sync: bool,
    #[arg(long)]
    pub client_id: String,
    #[arg(long)]
    pub client_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    let connector = HttpsConnector::new();
    let client = Client::builder().build(connector);
    let args = Args::parse();
    let dirs: BaseDirectories = xdg::BaseDirectories::with_prefix("strava-rs").unwrap();
    let access_token_path = dirs
        .place_state_file("access_token.json")
        .expect("Could not create state directory");
    let storage_path = dirs.get_data_home();
    let mut authenticator = Authenticator::new(
        client,
        args.client_id,
        args.client_secret,
        access_token_path.to_str().unwrap().to_string(),
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

    if args.no_sync != true {
        StravaSync::new(&client, &mut activity_store).sync().await?;
    }

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
    enable_raw_mode()?;
    terminal.clear()?;

    let layout = AppLayout::new(
        ActivityList::new(activity_store),
    );
    App::new(layout).run(&mut terminal)?;

    disable_raw_mode()?;

    Ok(())
}
