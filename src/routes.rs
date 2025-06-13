use crate::database::{query_all_articles, query_articles_by_source, return_json};

use axum::{Router, extract::Path, extract::State, routing::get};
use sqlx::PgPool;
use std::sync::Arc;

pub fn create_routes() -> Router<Arc<PgPool>> {
    Router::new()
        .route("/", get(home))
        .route("/api/articles/all", get(get_all_articles))
        .route("/api/articles/bysource/{id}", get(get_articles_by_source))
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

async fn get_articles_by_source(
    Path(id): Path<i64>,
    State(pool): State<Arc<PgPool>>,
) -> Result<String, (axum::http::StatusCode, String)> {
    match query_articles_by_source(id, &pool).await {
        Ok(articles) => Ok(return_json(articles).await.unwrap()),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
