mod models;

use axum::{Router, routing::get};
use chrono::NaiveDateTime;
use dotenv::dotenv;
use models::Article;
use sqlx::Error;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().expect("Failed to load .env file");

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");

    // above will crash the program if the file isnt found or if the file does not have the proper
    // env variable since we rely on em
    /* If we where not reliant on the env file we could do whats below to just print a warning instead
    if let Err(err) = dotenv() {
        eprintln!("Warning: .env not loaded: {}", err);
    }
    */

    // connects to the database and opens up a pool of 10 allowed connections
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    let result: i32 = sqlx::query_scalar("SELECT 1").fetch_one(&pool).await?;

    if result == 1 {
        println!("Database connection is alive.");
    } else {
        println!("Database connection is dead.");
    }

    let articles: Vec<Article> = get_all_articles(&pool).await?;
    for article in articles {
        println!("{}", article.title);
    }

    // Build the Axum app
    let app = Router::new().route("/", get("Hello World!"));

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_all_articles(pool: &PgPool) -> Result<Vec<Article>, sqlx::Error> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT id, title, url, source_id, published_at, content, summary, rating, processed, created_at FROM articles.arts"
    )
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

// TODO: get all articles from db and turn into article structs
// TODO: turn all articles into json
// TODO: send over api/articles/ or something similar
