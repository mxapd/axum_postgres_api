use chrono::NaiveDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub source_id: Option<i32>,
    pub published_at: Option<NaiveDateTime>,
    pub content: String,
    pub summary: Option<String>,
    pub rating: Option<String>,
    pub processed: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
}
