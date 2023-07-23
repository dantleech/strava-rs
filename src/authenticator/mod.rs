mod access_token_fetcher;
mod auth_code_fetcher;
mod token_store;
use std::time::{SystemTime, UNIX_EPOCH};

use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

use hyper::{client::HttpConnector, Client};

use crate::event::{logger::Logger};

use self::{
    access_token_fetcher::AccessTokenFetcher, auth_code_fetcher::AuthCodeFetcher,
    token_store::TokenStore,
};

pub struct Authenticator {
    token_fetch: AuthCodeFetcher,
    token_store: TokenStore,
    access_token_fetcher: AccessTokenFetcher,
    logger: Logger,
}

impl Authenticator {
    pub(crate) fn new(
        client: Client<HttpsConnector<HttpConnector>>,
        client_id: String,
        client_secret: String,
        token_path: String,
        logger: Logger,
    ) -> Self {
        Authenticator {
            token_fetch: AuthCodeFetcher::new(client_id.clone(), logger.clone()),
            token_store: TokenStore::new(token_path),
            access_token_fetcher: AccessTokenFetcher::new(client, client_id, client_secret),
            logger
        }
    }

    pub(crate) async fn access_token(&mut self) -> Result<String, anyhow::Error> {
        let token = self.token_store.get()?;
        if let Some(result) = token {
            if result.is_valid() {
                return Ok(result.access_token);
            }
        }

        self.logger.info("Authenticating".to_string()).await;
        let auth_code = self.token_fetch.auth_code().await?;
        let access_token: AuthResponse = self.access_token_fetcher.access_token(auth_code).await?;

        self.token_store.put(&access_token)?;

        Ok(access_token.access_token)
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

impl AuthResponse {
    pub(crate) fn is_valid(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("foo")
            .as_secs();
        self.expires_at >= now
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Athlete {
    pub id: u64,
    pub username: Option<String>,
    pub firstname: String,
    pub lastname: String,
}

#[cfg(test)]
mod test {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::AuthResponse;

    #[test]
    pub fn auth_response_expires() {
        let mut resp = auth_response();

        resp.expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("foo")
            .as_secs()
            + 10;

        assert!(resp.is_valid());

        resp.expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("foo")
            .as_secs()
            - 10;

        assert!(!resp.is_valid());
    }

    fn auth_response() -> AuthResponse {
        AuthResponse {
            expires_at: 123,
            expires_in: 0,
            token_type: "tokentype".to_string(),
            refresh_token: "refresh".to_string(),
            access_token: "token".to_string(),
            athlete: super::Athlete {
                id: 123,
                username: Some("dan".to_string()),
                firstname: "Dan".to_string(),
                lastname: "Leech".to_string(),
            },
        }
    }
}
