pub mod app;
pub mod authenticator;
pub mod client;
pub mod component;
pub mod event;
pub mod store;
pub mod sync;
pub mod ui;
pub mod util;

use std::{io, panic, process, sync::Arc, thread, time::Duration};

use app::App;
use authenticator::Authenticator;
use clap::Parser;
use client::{new_strava_client, StravaConfig};
use crossterm::event::{self as crossevent};
use crossterm::{
    event::{poll, Event},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use futures_util::future::UnwrapOrElse;
use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio::{
    sync::mpsc::{self, Sender},
    task,
};
use tui::{backend::CrosstermBackend, Terminal};
use xdg::BaseDirectories;

use crate::{
    store::activity::ActivityStore,
    sync::{convert::AcitivityConverter, ingest_activities::IngestActivitiesTask},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pub activity_type: Option<String>,
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
    let pool = Pool::builder().build(ConnectionManager::<SqliteConnection>::new(
        "sqlite://strava.sqlite",
    ))?;
    let (logger, log_receiver) = mpsc::channel(32);
    let (event_sender, event_receiver) = mpsc::channel(32);
    let logger = Arc::new(logger);

    log::info!("Strava TUI");
    log::info!("==========");
    log::info!("");
    log::info!("Token path: {}", access_token_path.display());
    log::info!("Storage path: {}", storage_path.display());
    log::info!("");

    if !args.no_sync {
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
        {
            let mut sync_conn = pool.clone().get().unwrap();
            task::spawn(async move {
                let client = new_strava_client(api_config, logger.clone());
                IngestActivitiesTask::new(&client, &mut sync_conn, logger.clone())
                    .execute()
                    .await
                    .unwrap();
                AcitivityConverter::new(&mut sync_conn, logger.clone())
                    .convert()
                    .await
                    .unwrap();
                Ok::<(), anyhow::Error>(())
            });
        }
    }

    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        orig_hook(panic_info);
        process::exit(1);
    }));

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
    enable_raw_mode()?;
    terminal.clear()?;

    // IO thread
    {
        thread::spawn(move || {
            loop {
                if poll(Duration::from_millis(10)).unwrap() {
                    if let Event::Key(key) = crossevent::read().unwrap() {
                        event_sender.blocking_send(key).unwrap();
                    }
                }
            }
        });
    }

    let mut app_conn = pool.clone().get().unwrap();
    let mut activity_store = ActivityStore::new(&mut app_conn);
    let mut app = App::new(&mut activity_store, log_receiver, event_receiver);
    app.activity_type = args.activity_type;
    app.run(&mut terminal).await?;

    disable_raw_mode()?;

    Ok(())
}
