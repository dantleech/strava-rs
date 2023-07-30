use confy::get_configuration_file_path;
use serde::{Deserialize, Serialize};
use clap::Parser;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub activity_type: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pub activity_type: Option<String>,
    #[arg(long)]
    pub client_id: Option<String>,
    #[arg(long)]
    pub client_secret: Option<String>,
}

pub struct Config {
    pub activity_type: Option<String>,
    pub client_id: String,
    pub client_secret: String,
}

pub enum ConfigResult {
    Ok(Config),
    Instructions(String)
}

pub fn load_config() -> ConfigResult {
    let args = Args::parse();
    let config: ConfigFile = confy::load("strava-rs", "config").expect("Could not load config");

    if None == config.client_id {
        return ConfigResult::Instructions(format!("
Welcome to Strava RS!
---------------------

Before you can use the app you need to setup your client and secret in Strava.

1) Visit: https://www.strava.com/settings/api
2) Create a new application (the callback domain can be anything)

Now configure the application in the following file:

    # {}
    client_id = \"your client id\"
    client_secret = \"your client secret\"

And run the application again, alternatively you can use the `--client-id` and `--client-secret` options!
                                   ", get_configuration_file_path("strava-rs", "config").unwrap().to_str().unwrap()))
    }

    ConfigResult::Ok(Config {
        activity_type: args.activity_type.or(config.activity_type),
        client_id: args.client_id.or(config.client_id).unwrap(),
        client_secret:args.client_secret.or(config.client_secret).unwrap() 
    })
}
