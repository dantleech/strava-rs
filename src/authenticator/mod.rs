use std::{convert::Infallible, fmt::format, net::SocketAddr, str::FromStr};
use hyper_tls::HttpsConnector;
use serde::{Serialize, Deserialize};
use tokio::sync::mpsc::channel;
use url::form_urlencoded;

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, Client, client::HttpConnector,
};

pub struct Authenticator {
    client: Client<HttpsConnector<HttpConnector>>,
    client_id: String,
    client_secret: String,
    addr: String,
}

impl Authenticator {
    pub(crate) fn new(
        client: Client<HttpsConnector<HttpConnector>>,
        client_id: String,
        client_secret: String
    ) -> Authenticator {
        Authenticator {
            client,
            client_id,
            client_secret,
            addr: "127.0.0.1:8112".to_string(),
        }
    }

    pub(crate) async fn access_token(&mut self) -> Result<String, anyhow::Error> {
        let (tx, mut rx) = channel::<String>(1);

        let addr = SocketAddr::from_str("127.0.0.1:8112").unwrap();

        println!("https://www.strava.com/oauth/authorize?client_id={}&response_type=code&redirect_uri=http://{}/exchange_token&approval_prompt=force&scope=activity:read_all,read", self.client_id, self.addr);

        let make_svc = make_service_fn(|_con| {
            let tx = tx.clone();

            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let tx = tx.clone();
                    async move {
                        let mut code = "".to_owned();

                        if let Some(query) = req.uri().query() {
                            for (k, v) in form_urlencoded::parse(query.as_bytes()) {
                                if k == "code" {
                                    code = v.into_owned();
                                }
                            }
                        }

                        tx.send(code.clone()).await.unwrap();
                        Ok::<Response<Body>, Infallible>(Response::new(Body::from(code)))
                    }
                }))
            }
        });

        let (tx1, mut rx1) = channel::<String>(1);

        let server = Server::bind(&addr)
            .serve(make_svc)
            .with_graceful_shutdown(async {
                let code = rx.recv().await;
                tx1.send(code.unwrap()).await.unwrap();
            });

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }

        let code = rx1.recv().await.unwrap();
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
            return Err(anyhow::Error::msg(format!("Got {} respponse for auth response", res.status())));
        }

        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let deserialized: AuthResponse  = serde_json::from_slice(&bytes)?;

        return Ok(deserialized.access_token);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub access_token: String,
}
