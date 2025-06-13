use crate::models::Article;
use sqlx::{Error, PgPool};

pub async fn query_all_articles(pool: &PgPool) -> Result<Vec<Article>, sqlx::Error> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT id, title, url, source_id, published_at, content, summary, rating, processed, created_at FROM articles.arts"
    )
    .fetch_all(pool)
    .await?;
    Ok(articles)
}

pub async fn return_json(articles: Vec<Article>) -> Result<String, Error> {
    let mut articles_json: String = String::new();
    for article in articles {
        let article_json: String = serde_json::to_string(&article).unwrap();
        articles_json = articles_json + &article_json + "\n\n";
    }
    Ok(articles_json)
}

pub async fn query_articles_by_source(
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
