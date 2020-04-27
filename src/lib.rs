#![warn(rust_2018_idioms)]

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::Mutex;

pub mod wasm;

pub trait Admit {
    fn admit<B>(&mut self, req: &http::Request<B>) -> bool;
}

#[derive(Debug)]
pub struct AdmitProxy<A> {
    client: hyper::Client<hyper::client::HttpConnector>,
    admit: Arc<Mutex<A>>,
}

impl<A> AdmitProxy<A> {
    pub fn new(client: hyper::Client<hyper::client::HttpConnector>, admit: A) -> Self {
        let admit = Arc::new(Mutex::new(admit));
        Self { client, admit }
    }
}

impl<A> Clone for AdmitProxy<A> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            admit: self.admit.clone(),
        }
    }
}

impl<A: Admit + Send + 'static> tower_service::Service<http::Request<hyper::Body>>
    for AdmitProxy<A>
{
    type Response = http::Response<hyper::Body>;
    type Error = hyper::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), hyper::Error>> {
        self.client.poll_ready(cx)
    }

    fn call(&mut self, req: http::Request<hyper::Body>) -> Self::Future {
        Box::pin(respond(req, self.admit.clone(), self.client.clone()))
    }
}

async fn respond<A: Admit>(
    req: http::Request<hyper::Body>,
    admit: Arc<Mutex<A>>,
    client: hyper::Client<hyper::client::HttpConnector>,
) -> Result<http::Response<hyper::Body>, hyper::Error> {
    if !admit.lock().await.admit(&req) {
        let mut rsp = http::Response::new(Default::default());
        *rsp.status_mut() = http::StatusCode::FORBIDDEN;
        Ok(rsp)
    } else {
        client.request(req).await
    }
}

impl Admit for bool {
    fn admit<B>(&mut self, _: &http::Request<B>) -> bool {
        *self
    }
}
