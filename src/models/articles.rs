use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct Article {
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub html: String,
    pub views: i32,
    pub comment_count: i32,
    pub allow_comment: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticle {
    pub title: String,
    pub description: String,
    pub content: String,
}

impl CreateArticle {
    pub fn new(title: String, description: String, content: String) -> CreateArticle {
        CreateArticle {
            title,
            description,
            content,
        }
    }
}