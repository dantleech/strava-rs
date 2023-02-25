pub mod client;
pub mod authenticator;


use clap::Parser;
use client::{new_strava_client, StravaConfig};
use authenticator::Authenticator;
use hyper::Client;
use hyper_tls::HttpsConnector;
use xdg::BaseDirectories;

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

    log::info!("Strava TUI");
    log::info!("==========");
    log::info!("");
    log::info!("Token path: {}", access_token_path.display());
    log::info!("");

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
    let client = new_strava_client(api_config);

    println!("{:#?}", client.athlete_activities().await?);
    Ok(())
}
