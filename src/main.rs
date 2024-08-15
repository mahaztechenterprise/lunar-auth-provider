use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let ip = [127, 0, 0, 1];
    let port = 8080;

    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from((ip, port));

    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service());
    
    println!("Server started {:#?}:{:#?}", ip, port);

    server.await.unwrap();
}

async fn handler() -> &'static str {
    "Hello, world!"
}