use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use dotenvy::dotenv;
use std::env;

pub async fn init_db() -> SqlitePool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database")
}
