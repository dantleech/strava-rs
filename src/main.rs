pub mod app;
pub mod authenticator;
pub mod expr;
pub mod client;
pub mod component;
pub mod config;
pub mod event;
pub mod store;
pub mod sync;
pub mod ui;
pub mod util;

use std::{io, panic, process};

use app::App;
use config::ConfigResult;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use event::input;

use log::info;
use tokio::sync::mpsc::{self};
use tui::{backend::CrosstermBackend, Terminal};
use tui_logger::{init_logger, set_default_level};
use xdg::BaseDirectories;

use crate::store::activity::ActivityStore;
use crate::{
    config::{load_config, Config},
    event::logger::Logger,
    store::{db::get_pool, migration::run_migrations},
    sync::spawn_sync,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init_logger(log::LevelFilter::Trace)?;
    set_default_level(log::LevelFilter::Trace);

    let dirs: BaseDirectories = xdg::BaseDirectories::with_prefix("strava-rs").unwrap();
    let access_token_path = dirs
        .place_state_file("access_token.json")
        .expect("Could not create state directory");
    let storage_path = dirs
        .create_data_directory("")
        .expect("Could not create data directory");
    let pool = get_pool(format!("{}/strava.sqlite", storage_path.display())).await;
    let (event_sender, event_receiver) = mpsc::channel(32);
    let (sync_sender, sync_receiver) = mpsc::channel::<bool>(32);
    let logger = Logger::new(event_sender.clone());

    let config_result = load_config();
    let config: Config = match config_result {
        ConfigResult::Ok(c) => c,
        ConfigResult::Instructions(m) => {
            println!("{}", m);
            return Ok(());
        }
    };

    log::info!("Strava TUI");
    log::info!("==========");
    log::info!("");
    log::info!("Token path: {}", access_token_path.display());
    log::info!("Storage path: {}", storage_path.display());
    log::info!("");

    run_migrations(&pool).await;

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
        config.client_id,
        config.client_secret,
        access_token_path.to_str().unwrap().to_string(),
        logger,
        sync_receiver,
    )
    .await;

    let mut activity_store = ActivityStore::new(&pool);
    let mut app = App::new(
        &mut activity_store,
        event_receiver,
        event_sender.clone(),
        sync_sender,
    );
    app.send(input::InputEvent::Reload);
    app.send(input::InputEvent::Reload);
    app.activity_type = config.activity_type;
    info!("Starting application");
    app.run(&mut terminal).await?;
    sync_task.abort();

    disable_raw_mode()?;

    Ok(())
}
