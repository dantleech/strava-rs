use hyper::{
    service::{make_service_fn, service_fn},
    Body, Response, Server,
};
use std::{convert::Infallible, net::SocketAddr, str::FromStr};

use tokio::sync::mpsc::channel;
use url::form_urlencoded;

use crate::event::logger::Logger;

pub struct AuthCodeFetcher {
    client_id: String,
    addr: String,
    logger: Logger,
}

impl AuthCodeFetcher {
    pub(crate) fn new(client_id: String, logger: Logger) -> Self {
        Self {
            client_id,
            addr: "127.0.0.1:8112".to_string(),
            logger,
        }
    }

    #[allow(deprecated)]
    pub(crate) async fn auth_code(&mut self) -> Result<String, anyhow::Error> {
        let (tx, mut rx) = channel::<String>(1);

        let addr = SocketAddr::from_str("127.0.0.1:8112").unwrap();

        let auth_url = format!("https://www.strava.com/oauth/authorize?client_id={}&response_type=code&redirect_uri=http://{}/exchange_token&approval_prompt=force&scope=activity:read_all,read", self.client_id, self.addr);

        self.logger.info(format!("Trying to open URL: {}", auth_url)).await;

        // the alternative is blocking and does not start the webserver
        open::that_in_background(&auth_url);

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
                        Ok::<Response<Body>, Infallible>(Response::new(Body::from(
                            "
                        <html>
                            <head><title>Rust Authentication</title></head>
                            <body>
                                <h1>Strava TUI - access granted</h1>
                                <p>Close this window and return to your terminal</p>
                            </body>
                        </html>
                        ",
                        )))
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

        log::info!(
            "starting web server at {} and listening for auth code",
            self.addr
        );
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }

        Ok(rx1.recv().await.unwrap())
    }
}
