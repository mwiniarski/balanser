use std::net::SocketAddr;

use hyper::client::connect::HttpConnector;
use hyper::service::{Service};
use hyper::{Body, Request, Response, Server, Client, Uri};

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/*
1. Implement Service << DONE >>

2. Implement round-robin

3. Add external config (WOJT)

4. Integrate external config

Extra
*. Integration test

*/

const SERVERS: [&str; 3] = [
    "http://127.0.0.1:8000",
    "http://127.0.0.1:8001",
    "http://127.0.0.1:8002",
];

struct Svc {
    uri: Uri,
    client: Client<HttpConnector, Body>,
}

// Service implementation
impl Service<Request<Body>> for Svc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send >>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let uri = self.uri.clone();
        *req.uri_mut() = uri;
        println!("serving {}", req.uri());
        let fut = self.client.request(req);
        Box::pin(fut)
    }
}

struct MakeSvc {
    index: usize,
    client: Client<HttpConnector, Body>,
}

impl<T> Service<T> for MakeSvc {
    type Response = Svc;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let index = self.index.clone();
        let client = self.client.clone();
        let fut = async move { Ok(Svc { client,  uri: Uri::from_static(SERVERS[index]) }) };
        self.index = (self.index + 1) % SERVERS.len();
        Box::pin(fut)
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 7000));

    println!("Listening on {}...", addr);

    let client = Client::new();
    let server = Server::bind(&addr).serve(MakeSvc{client, index: 0});

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    // If you don't understand any of this code - learn rust and RTFM
}
