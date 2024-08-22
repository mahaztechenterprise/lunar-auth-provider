use std::str::FromStr;
use log::{info, error};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use refinery::config::Config;

pub struct PoolConnection {
    pub db: MySqlPool
}


mod embedded {
    refinery::embed_migrations!("./migrations/");
}

pub async fn connect() -> PoolConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("Connection to the database is successful!");
            info!("Migrating database");
            
            let conn = Config::from_str(&database_url);
            let result = embedded::migrations::runner().run(&mut conn.unwrap()).unwrap();
            info!("{:#?}", result);
            pool
        }
        Err(err) => {
            error!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    
    return PoolConnection { db: pool.clone() };
}