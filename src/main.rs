pub mod authenticator;
pub mod client;
pub mod store;
pub mod sync;
pub mod component;
pub mod util;

use std::io;

use authenticator::Authenticator;
use clap::Parser;
use client::{new_strava_client, StravaConfig};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use diesel::{Connection, SqliteConnection};
use hyper::Client;
use hyper_tls::HttpsConnector;
use tui::{backend::CrosstermBackend, Terminal};
use xdg::BaseDirectories;

use crate::{
    store::activity::ActivityStore,
    sync::{convert::AcitivityConverter, ingest::StravaSync},
    component::{
        activity_list::ActivityList,
        app::App,
        layout::{AppLayout, State, View},
    },
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
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
    let args = Args::parse();
    let dirs: BaseDirectories = xdg::BaseDirectories::with_prefix("strava-rs").unwrap();
    let access_token_path = dirs
        .place_state_file("access_token.json")
        .expect("Could not create state directory");
    let storage_path = dirs.get_data_home();
    let mut db = SqliteConnection::establish("sqlite://strava.sqlite")
        .expect("Could not connect to Sqlite database");

    log::info!("Strava TUI");
    log::info!("==========");
    log::info!("");
    log::info!("Token path: {}", access_token_path.display());
    log::info!("Storage path: {}", storage_path.display());
    log::info!("");

    if args.no_sync != true {
        let client = Client::builder().build(connector);
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
        let client = new_strava_client(api_config);
        StravaSync::new(&client, &mut db).sync().await?;
        AcitivityConverter::new(&mut db).convert().await?;
    }

    let mut activity_store = ActivityStore::new(&mut db);

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
    enable_raw_mode()?;
    terminal.clear()?;

    let mut state = State {
        view: View::ActivityList,
        activity: None,
    };
    let mut list = ActivityList::new(&mut activity_store);
    let mut layout = AppLayout::new(&mut list, state);
    App::new(&mut layout).run(&mut terminal)?;

    disable_raw_mode()?;

    Ok(())
}
