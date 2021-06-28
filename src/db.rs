use std::env;

use sqlx::SqlitePool;

pub async fn connect(database_url: Option<&str>) -> Result<sqlx::Pool<sqlx::Sqlite>, sqlx::Error> {
    let env_url = env::var("DATABASE_URL").unwrap_or(
        dotenv::var("DATABASE_URL")
            .expect("Missing environment variable DATABASE_URL, please set it."),
    );
    let url = database_url.unwrap_or(&env_url);
    return SqlitePool::connect(url).await;
}
