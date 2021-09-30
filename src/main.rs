use std::net::SocketAddr;
use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, Client, Uri};

mod config;

const SERVERS: [&str; 4] = [
    "http://127.0.0.1:8000", // Only using this one for now
    "google.com",
    "glovoapp.com",
    "ea.com"
];

async fn handle(_: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Client::new().get(Uri::from_static(SERVERS[0])).await
}

#[tokio::main]
async fn main() {
    let configs = config::init();
    let listen_address = configs["listen_address"].as_str().unwrap();
    let addr: SocketAddr = listen_address.parse().expect("Unable to parse listen address");

    println!("Listening on {}...", addr);

    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    // If you don't understand any of this code - learn rust and RTFM
}
