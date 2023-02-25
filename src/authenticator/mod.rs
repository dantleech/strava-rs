mod access_token_fetcher;
mod auth_code_fetcher;
mod token_store;
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

use hyper::{
    client::HttpConnector, Client,
};

use self::{auth_code_fetcher::AuthCodeFetcher, token_store::TokenStore, access_token_fetcher::AccessTokenFetcher};

pub struct Authenticator {
    token_fetch: AuthCodeFetcher,
    token_store: TokenStore,
    access_token_fetcher: AccessTokenFetcher,
}

impl Authenticator {
    pub(crate) fn new(
        client: Client<HttpsConnector<HttpConnector>>,
        client_id: String,
        client_secret: String,
        token_path: String,
    ) -> Self {
        Authenticator {
            token_fetch: AuthCodeFetcher::new(client_id.clone()),
            token_store: TokenStore::new(token_path),
            access_token_fetcher: AccessTokenFetcher::new(client, client_id, client_secret)
        }
    }
    pub(crate) async fn access_token(&mut self) -> Result<String, anyhow::Error> {
        let token = self.token_store.get()?;
        if let Some(result) = token {
            return Ok(result.access_token)
        }
        let auth_code = self.token_fetch.auth_code().await?;
        let access_token: AuthResponse = self.access_token_fetcher.access_token(auth_code).await?;
        self.token_store.put(&access_token)?;

        return Ok(access_token.access_token);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub token_type: String,
    pub expires_at: u64,
    pub expires_in: u64,
    pub refresh_token: String,
    pub access_token: String,
    pub athlete: Athlete,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Athlete {
    pub id: u64,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
}
