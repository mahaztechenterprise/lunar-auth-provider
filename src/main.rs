use std::net::SocketAddr;

mod routes;
use crate::routes::app_route::create_routes;

#[tokio::main]
async fn main() {
    let routes = create_routes();

    let ip = [127, 0, 0, 1];
    let port = 8080;

    let addr = SocketAddr::from((ip, port));

    let server = axum::Server::bind(&addr)
        .serve(routes.into_make_service());
    
    println!("Server started {:#?}:{:#?}", ip, port);

    server.await.unwrap();
}