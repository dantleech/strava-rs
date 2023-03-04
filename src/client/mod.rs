#![allow(dead_code)]

use std::fmt::Display;

use hyper::{client::HttpConnector, Body, Client, Method, Request, Response};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub fn new_strava_client(config: StravaConfig) -> StravaClient {
    let connector = HttpsConnector::new();
    let client = Client::builder().build(connector);

    StravaClient {
        config,
        client,
        access_token: None,
    }
}

#[derive(Debug)]
pub struct StravaConfig {
    pub base_url: String,
    pub access_token: String,
}

pub struct StravaClient {
    client: Client<HttpsConnector<HttpConnector>>,
    config: StravaConfig,
    access_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    pub id: u64,
    pub name: String,
    pub distance: f64,
    pub moving_time: u64,
    pub elapsed_time: u64,
    pub total_elevation_gain: f64,
    pub sport_type: String,
}

impl Display for Activity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}] {}", self.id, self.sport_type, self.name)
    }
}

impl StravaClient {
    async fn request<T>(&self, method: Method, path: String) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.config.base_url, path);
        let req = Request::builder()
            .uri(&url)
            .method(method)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.access_token),
            )
            .body(Body::default())
            .unwrap();

        let res: Response<Body> = self.client.request(req).await?;

        if res.status() != 200 {
            return Err(anyhow::Error::msg(format!(
                "Got {} respponse for URL {}",
                res.status(),
                &url
            )));
        }

        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let deserialized: T = serde_json::from_slice(&bytes)?;

        Ok(deserialized)
    }

    pub async fn athlete_activities(&self) -> Result<Vec<Activity>, anyhow::Error> {
        let activity = self
            .request(Method::GET, "/v3/athlete/activities".to_string())
            .await?;

        Ok(activity)
    }
}
