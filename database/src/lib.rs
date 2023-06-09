pub mod addresses;
pub mod chats;
pub mod companies;
pub mod projects;
pub mod users;
pub mod utils;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

pub async fn pg_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().expect("failed to load .env");

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(1))
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be in environment"))
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_can_create_a_pg_pool() {
        assert!(pg_pool().await.is_ok());
    }
}
