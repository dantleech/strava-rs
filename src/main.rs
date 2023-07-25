pub mod app;
pub mod authenticator;
pub mod client;
pub mod component;
pub mod event;
pub mod store;
pub mod sync;
pub mod ui;
pub mod util;

use std::{io, panic, process, ops::DerefMut};


use app::App;

use clap::Parser;


use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
};

use event::input;
use tokio::{
    sync::mpsc::{self},
};
use tui::{backend::CrosstermBackend, Terminal};
use xdg::BaseDirectories;

use crate::{sync::{spawn_sync}, store::{db::get_pool, migration::run_migrations}, event::logger::Logger};
use crate::{
    store::activity::ActivityStore,
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

    let args = Args::parse();
    let dirs: BaseDirectories = xdg::BaseDirectories::with_prefix("strava-rs").unwrap();
    let access_token_path = dirs
        .place_state_file("access_token.json")
        .expect("Could not create state directory");
    let storage_path = dirs.get_data_home();
    let pool = get_pool(format!("{}/strava.sqlite", storage_path.display()));
    let (event_sender, event_receiver) = mpsc::channel(32);
    let (sync_sender, sync_receiver) = mpsc::channel::<bool>(32);
    let logger = Logger::new(event_sender.clone());

    log::info!("Strava TUI");
    log::info!("==========");
    log::info!("");
    log::info!("Token path: {}", access_token_path.display());
    log::info!("Storage path: {}", storage_path.display());
    log::info!("");

    let mut c = pool.get()?;
    run_migrations(c.deref_mut());

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

    // start input thread
    input::start(event_sender.clone());

    // start sync async task
    let sync_task = spawn_sync(
        pool.clone(),
        event_sender.clone(),
        args.client_id,
        args.client_secret,
        access_token_path.to_str().unwrap().to_string(),
        logger,
        sync_receiver
    ).await;

    let mut app_conn = pool.clone().get().unwrap();
    let mut activity_store = ActivityStore::new(&mut app_conn);
    let mut app = App::new(&mut activity_store, event_receiver, event_sender.clone(), sync_sender);
    app.activity_type = args.activity_type;
    app.send(input::InputEvent::Sync);
    app.run(&mut terminal).await?;
    sync_task.abort();

    disable_raw_mode()?;

    Ok(())
}
