use std::sync::Arc;
use dotenv::dotenv;
use sqlx::Connection;
use tokio::net::TcpListener;

mod user;
mod database;
use crate::user::routes::app_route::create_routes;
use crate::database::configuration::mysql_db_config;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_type = std::env::var("DATABASE_TYPE")
        .unwrap_or(String::from("MySQL"));

    println!("Database type set to {:#?}", database_type);


    let pool = mysql_db_config::connect().await;

    let routes = create_routes(Arc::new(pool));

    let ip = "127.0.0.1:8080";

    let addr = TcpListener::bind(ip).await.unwrap();

    let server = axum::serve(addr, routes);
    
    println!("Server started {:#?}", ip);

    server.await.unwrap();
}