extern crate airlinepro;
use airlinepro::login;
use sqlx::sqlite::SqlitePool; // Import SqlitePool
use std::env;

#[tokio::main] // Use the tokio runtime for async main
async fn main() {
    // Load environment variables (optional)
    dotenv::dotenv().ok();

    // Database URL, e.g., "sqlite://airline.db"
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to create a pool.");

    // Call userinterface function with the pool
    login::userinterface(&pool).await;
}
