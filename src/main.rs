pub mod client;
pub mod authenticator;

use anyhow::Error;
use clap::Parser;
use client::{new_strava_client, StravaConfig};
use authenticator::Authenticator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    pub client_id: String,
    #[arg(long)]
    pub client_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), Error>{
    let args = Args::parse();
    let mut authenticator = Authenticator::new(
        args.client_id,
        args.client_secret,
    );

    let api_config = StravaConfig {
        base_url: "https://www.strava.com/api".to_string(),
        access_token: authenticator.access_token().await?,
    };
    let client = new_strava_client(api_config);

    client.athlete_activities().await?;
    Ok(())
}
