#![warn(rust_2018_idioms)]

use std::env;
use std::task::{Context, Poll};
//use wasmer_runtime as wasm;

#[tokio::main]
async fn main() {
    let listen_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8000".to_string())
        .parse()
        .unwrap();

    let _header = env::args()
        .nth(2)
        .unwrap_or_else(|| "waxy-admit".to_owned());

    let client = hyper::Client::new();

    let server = hyper::Server::bind(&listen_addr).serve({
        let client = client.clone();
        hyper::service::make_service_fn(move |_| {
            let client = client.clone();
            async move { Ok::<_, hyper::Error>(AdmitProxy { client }) }
        })
    });

    println!("Listening on: {}", listen_addr);
    if let Err(e) = server.await {
        eprintln!("Server failed: {}", e);
    }
}

#[derive(Clone, Debug)]
struct AdmitProxy {
    client: hyper::Client<hyper::client::HttpConnector>,
}

impl tower_service::Service<http::Request<hyper::Body>> for AdmitProxy {
    type Response = http::Response<hyper::Body>;
    type Error = hyper::Error;
    type Future = hyper::client::ResponseFuture;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), hyper::Error>> {
        self.client.poll_ready(cx)
    }

    fn call(&mut self, req: http::Request<hyper::Body>) -> Self::Future {
        println!("Sending request: {:?}", req.uri());
        self.client.call(req)
    }
}
