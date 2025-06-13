mod models;
use axum::extract::Path;
use axum::{Router, extract::State, response::Json, routing::get};
use dotenv::dotenv;
use models::Article;
use sqlx::Error;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;

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
    let shared_pool = Arc::new(pool);
    // Build the Axum app
    let app = Router::new()
        .route("/", get(home))
        .route("/api/articles/all", get(get_all_articles))
        .route("/api/articles/bysource/{id}", get(get_articles_by_source))
        .with_state(shared_pool);
    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn home() -> &'static str {
    "Home"
}

async fn get_all_articles(
    State(pool): State<Arc<PgPool>>,
) -> Result<String, (axum::http::StatusCode, String)> {
    match query_all_articles(&pool).await {
        Ok(articles) => Ok(return_json(articles).await.unwrap()),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn query_all_articles(pool: &PgPool) -> Result<Vec<Article>, sqlx::Error> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT id, title, url, source_id, published_at, content, summary, rating, processed, created_at FROM articles.arts"
    )
    .fetch_all(pool)
    .await?;
    Ok(articles)
}

async fn return_json(articles: Vec<Article>) -> Result<String, Error> {
    let mut articles_json: String = String::new();
    for article in articles {
        let article_json: String = serde_json::to_string(&article).unwrap();
        //println!("{} : JSON:/n {}", article.title, article_json);
        articles_json = articles_json + &article_json + "\n\n";
    }
    Ok(articles_json)
}

async fn query_articles_by_source(
    source_id: i64,
    pool: &PgPool,
) -> Result<Vec<Article>, sqlx::Error> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT id, title, url, source_id, published_at, content, summary, rating, processed, created_at FROM articles.arts WHERE source_id=$1"
    ).bind(source_id)
    .fetch_all(pool)
    .await?;
    Ok(articles)
}

async fn get_articles_by_source(
    Path(id): Path<i64>,
    State(pool): State<Arc<PgPool>>,
) -> Result<String, (axum::http::StatusCode, String)> {
    match query_articles_by_source(id, &pool).await {
        Ok(articles) => Ok(return_json(articles).await.unwrap()),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
