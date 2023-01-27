#![allow(dead_code)]

use std::fmt::format;

use hyper::{Body, Client, Method, Request, Response, client::HttpConnector};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub fn new_strava_client(config: StravaConfig) -> StravaClient {
    let connector = HttpsConnector::new();
    let client = Client::builder().build(connector);

    StravaClient { config, client, access_token: None }
}

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
    pub name: String,
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
            .header("Authorization", format!("Bearer: {}", self.config.access_token))
            .body(Body::default())
            .unwrap();

        let res: Response<Body> = self.client.request(req).await?;

        if res.status() != 200 {
            return Err(anyhow::Error::msg(format!("Got {} respponse for URL {}", res.status(), &url)));
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
