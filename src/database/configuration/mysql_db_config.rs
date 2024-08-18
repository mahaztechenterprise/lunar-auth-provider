use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub struct PoolConnection {
    pub db: MySqlPool
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
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    return PoolConnection { db: pool.clone() };
}