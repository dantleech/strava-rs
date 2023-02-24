use std::{convert::Infallible, net::SocketAddr, str::FromStr};
use tokio::sync::mpsc::channel;
use url::form_urlencoded;

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Response, Server,
};

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
        let (tx, mut rx) = channel::<String>(1);

        let addr = SocketAddr::from_str("127.0.0.1:8112").unwrap();


        println!("https://www.strava.com/oauth/authorize?client_id={}&response_type=code&redirect_uri=http://{}/exchange_token&approval_prompt=force&scope=read", self.client_id, self.addr);

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

        let server = Server::bind(&addr)
            .serve(make_svc)
            .with_graceful_shutdown(async {
                let code = rx.recv().await;
                println!("{}", code.unwrap())
            });

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }

        return Ok("".to_string());
    }
}
