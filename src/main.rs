pub mod client;

use clap::Parser;
use client::{new_strava_client, StravaClient, StravaConfig};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    pub access_token: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let api_config = StravaConfig {
        base_url: "https://www.strava.com/api".to_string(),
        access_token: args.access_token,
    };
    let client = new_strava_client(api_config);

    client.athlete_activities().await.unwrap();
}
