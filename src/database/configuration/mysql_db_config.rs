use std::{borrow::BorrowMut, fmt::Debug, str::FromStr};

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
            println!("Connection to the database is successful!");
            println!("Migrating database");
            
            let conn = Config::from_str(&database_url);
            let result = embedded::migrations::runner().run(&mut conn.unwrap()).unwrap();
            println!("{:#?}", result);
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    
    return PoolConnection { db: pool.clone() };
}