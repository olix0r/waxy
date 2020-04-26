#![warn(rust_2018_idioms)]

use std::env;
use waxy::AdmitProxy;

#[tokio::main]
async fn main() {
    // let listen_addr = env::args()
    //     .nth(1)
    //     .unwrap_or_else(|| "127.0.0.1:8000".to_string())
    //     .parse()
    //     .unwrap();
    let listen_addr = "127.0.0.1:8000".parse().unwrap();

    let wasm_path = env::args().nth(1).expect("usage: waxy WASM [HEADER]");
    let wasm = tokio::fs::read(wasm_path)
        .await
        .expect("Failed to read WASM file");

    let header = env::args()
        .nth(2)
        .unwrap_or_else(|| "waxy-admit".to_owned());

    let wasm_admit = waxy::wasm::AdmitHeader::new(&wasm, header).expect("Failed to load WASM");
    let admit = AdmitProxy::new(hyper::Client::new(), wasm_admit);

    let server = hyper::Server::bind(&listen_addr).serve({
        let admit = admit.clone();
        hyper::service::make_service_fn(move |_| {
            let admit = admit.clone();
            async move { Ok::<_, hyper::Error>(admit) }
        })
    });

    println!("Listening on: {}", listen_addr);
    if let Err(e) = server.await {
        eprintln!("Server failed: {}", e);
    }
}
