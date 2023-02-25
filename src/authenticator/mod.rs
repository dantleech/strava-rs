mod token_fetch;
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, fmt::format, net::SocketAddr, str::FromStr};
use tokio::sync::mpsc::channel;
use url::form_urlencoded;

use hyper::{
    client::HttpConnector,
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server,
};

use self::token_fetch::TokenFetch;

pub struct Authenticator {
    token_fetch: TokenFetch,
}

impl Authenticator {
    pub(crate) fn new(
        client: Client<HttpsConnector<HttpConnector>>,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Authenticator {
            token_fetch: TokenFetch::new(client, client_id, client_secret),
        }
    }
    pub(crate) async fn access_token(&mut self) -> Result<String, anyhow::Error> {
        self.token_fetch.access_token().await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub access_token: String,
}
