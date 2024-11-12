#![allow(dead_code)]

use std::fmt::Display;

use chrono::{DateTime, NaiveDateTime, Utc};
use hyper::{client::HttpConnector, Body, Client, Method, Request, Response};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::event::logger::Logger;

pub fn new_strava_client(config: StravaConfig, logger: Logger) -> StravaClient {
    let connector = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();
    let client = Client::builder().build(connector);

    StravaClient {
        config,
        client,
        access_token: None,
        logger,
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
    logger: Logger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub distance: f64,
    pub moving_time: i64,
    pub elapsed_time: i64,
    pub total_elevation_gain: f64,
    pub sport_type: String,
    pub average_heartrate: Option<f64>,
    pub max_heartrate: Option<f64>,
    pub start_date: Option<DateTime<Utc>>,
    pub map: Map,
    pub average_cadence: Option<f64>,
    pub average_speed: Option<f64>,
    pub kudos_count: i64,
    pub location_country: Option<String>,
    pub location_state: Option<String>,
    pub location_city: Option<String>,
    pub athlete_count: i64,
    pub splits_metric: Option<Vec<Split>>,
    pub splits_standard: Option<Vec<Split>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Split {
    pub distance: f64,
    pub moving_time: i64,
    pub elapsed_time: i64,
    pub average_speed: f64,
    pub elevation_difference: f64,
    pub split: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    pub summary_polyline: String,
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

        self.logger.info(format!(">> {}", url)).await;

        let res: Response<Body> = self.client.request(req).await?;

        if res.status() != 200 {
            let message = format!("Got {} response for URL {}", res.status(), &url);
            self.logger.error(message.clone()).await;
            return Err(anyhow::Error::msg(message));
        }

        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let deserialized: T = serde_json::from_slice(&bytes)?;

        Ok(deserialized)
    }

    pub async fn athlete_activities(
        &self,
        page: u32,
        per_page: u32,
        after: Option<NaiveDateTime>,
    ) -> Result<Vec<Value>, anyhow::Error> {
        let activities = self
            .request(
                Method::GET,
                format!(
                    "/v3/athlete/activities?per_page={}&page={}&after={}",
                    per_page,
                    page,
                    match after {
                        Some(epoch) => epoch.and_utc().timestamp().to_string(),
                        None => "".to_string(),
                    }
                ),
            )
            .await?;

        Ok(activities)
    }

    pub async fn athlete_activity(&self, id: String) -> Result<Value, anyhow::Error> {
        let activity = self
            .request(Method::GET, format!("/v3/activities/{}", id,))
            .await?;

        Ok(activity)
    }
}
