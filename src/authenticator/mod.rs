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
    ) -> Self {
        Authenticator {
            token_fetch: AuthCodeFetcher::new(client_id.clone()),
            token_store: TokenStore::new(),
            access_token_fetcher: AccessTokenFetcher::new(client, client_id, client_secret)
        }
    }
    pub(crate) async fn access_token(&mut self) -> Result<String, anyhow::Error> {
        if let Some(result) = self.token_store.get() {
            return Ok(result.access_token)
        }
        let auth_code = self.token_fetch.auth_code().await?;
        let access_token: AuthResponse = self.access_token_fetcher.access_token(auth_code).await?;

        return Ok(access_token.access_token);

    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub access_token: String,
}
