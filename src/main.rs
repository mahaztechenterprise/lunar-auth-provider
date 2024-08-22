use std::sync::Arc;
use dotenv::dotenv;
use tokio::net::TcpListener;
use log::info;

mod user;
mod database;
mod logger;

use crate::user::routes::app_route::create_routes;
use crate::database::configuration::mysql_db_config;
use crate::logger::log4rs::init;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init();

    let database_type = std::env::var("DATABASE_TYPE")
        .unwrap_or(String::from("MySQL"));

    info!(target: "database_type event", "Database type set to {database_type:?}");


    let pool = mysql_db_config::connect().await;

    let routes = create_routes(Arc::new(pool));

    let ip = "127.0.0.1:8080";

    let addr = TcpListener::bind(ip).await.unwrap();

    let server = axum::serve(addr, routes);
    
    info!(target: "server_started", "Server started {ip:?}");

    server.await.unwrap();
}