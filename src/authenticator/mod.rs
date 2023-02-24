use std::{convert::Infallible, net::SocketAddr, str::FromStr, thread};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use url::form_urlencoded;

use anyhow::Error;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use termion::color;

pub struct Authenticator {
    client_id: String,
    client_secret: String,
    addr: String,
}

impl Authenticator {
    pub(crate) fn new(client_id: String, client_secret: String) -> Authenticator {
        Authenticator {
            client_id,
            client_secret,
            addr: "127.0.0.1:8112".to_string(),
        }
    }

    pub(crate) async fn access_token(&mut self) -> Result<String, anyhow::Error> {
        let (tx, mut rx) = channel::<()>(1);

        let addr = SocketAddr::from_str("127.0.0.1:8112");

        format!("https://www.strava.com/oauth/authorize?client_id={}&response_type=code&redirect_uri=http://localhost/exchange_token&approval_prompt=force&scope=read", self.client_id);

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

                        tx.send(()).await.unwrap();
                        Ok::<Response<Body>, Infallible>(Response::new(Body::from(code)))
                    }
                }))
            }
        });

        let server = Server::bind(&addr)
            .serve(make_svc)
            .with_graceful_shutdown(async {
                rx.recv().await;
            });

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }

        return Ok("".to_string());
    }
}
