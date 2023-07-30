use hyper::{client::HttpConnector, Body, Client, Request, Response};
use hyper_rustls::HttpsConnector;

use super::AuthResponse;

pub struct AccessTokenFetcher {
    client: Client<HttpsConnector<HttpConnector>>,
    client_id: String,
    client_secret: String,
}

impl AccessTokenFetcher {
    pub(crate) fn new(
        client: Client<HttpsConnector<HttpConnector>>,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Self {
            client,
            client_id,
            client_secret,
        }
    }

    pub(crate) async fn access_token(
        &mut self,
        code: String,
    ) -> Result<AuthResponse, anyhow::Error> {
        let req = Request::builder()
            .uri("https://www.strava.com/oauth/token")
            .method("POST")
            .body(Body::from(format!(
                "client_id={}&client_secret={}&code={}&grant_type=authorization_code",
                self.client_id, self.client_secret, code
            )))
            .unwrap();

        let res: Response<Body> = self.client.request(req).await?;

        if res.status() != 200 {
            return Err(anyhow::Error::msg(format!(
                "Got '{}' when trying to authenticate, check your client ID and secret",
                res.status()
            )));
        }

        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let deserialized: AuthResponse = serde_json::from_slice(&bytes)?;

        Ok(deserialized)
    }
}
