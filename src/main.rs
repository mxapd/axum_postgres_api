mod database;
mod models;
mod routes;

use axum::Router;
use dotenv::dotenv;
use sqlx::Error;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().expect("Failed to load .env file");
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");
    // above will crash the program if the file isnt found or if the file does not have the proper
    // env variable since we rely on it

    // connects to the database and opens up a pool of 10 allowed connections
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    let shared_pool = Arc::new(pool);

    // Build the Axum app
    let app = Router::new()
        .merge(routes::create_routes())
        .with_state(shared_pool);

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
