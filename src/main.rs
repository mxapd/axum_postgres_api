use axum::{Router, routing::get};
use dotenv::dotenv;
use sqlx::Error;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect to the database
    let pool = connect_db().await?;

    // Build the Axum app
    let app = Router::new().route(
        "/",
        get(move || async move {
            match fetch_titles(&pool).await {
                Ok(titles) => get_titles(titles).await, // Pass Vec<String>
                Err(_) => "Error".to_string(),          // Handle failure
            }
        }),
    );

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn connect_db() -> Result<PgPool, Error> {
    dotenv().ok(); // Load environment variables

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");

    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Successfully connected to the database!");

    // Simple test query
    let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await?;
    println!("Query result: {}", row.0);

    Ok(pool)
}

async fn get_titles(titles: Vec<String>) -> String {
    let body = titles.join("\n");
    body
}

async fn fetch_titles(pool: &PgPool) -> Result<Vec<String>, Error> {
    let titles = sqlx::query_scalar("SELECT title FROM articles.arts")
        .fetch_all(pool)
        .await?;

    Ok(titles)
}
