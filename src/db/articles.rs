use sqlx::postgres::PgPool;

use crate::auth::AuthUser;
use crate::models::articles::{Article, CreateArticle};
use crate::utils::slugify;
use crate::{Error, Result};

pub async fn get_all_article_for_db(pool: &PgPool) -> Result<Vec<Article>> {
    let rows = sqlx::query_as!(Article, "SELECT * FROM articles")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn get_article_details_for_db(pool: &PgPool, slug: &str) -> Result<Article> {
    let row = sqlx::query_as!(Article, "SELECT * FROM articles WHERE slug = $1", slug)
        .fetch_optional(pool)
        .await?;

    if let Some(article) = row {
        Ok(article)
    } else {
        Err(Error::NotFound)
    }
}

pub async fn post_new_article_db(pool: &PgPool, user: AuthUser, article: CreateArticle) -> Result<Article> {
    let user_id = user.user_id;
    let slug = slugify(&article.title);
    let html = String::new();

    let row = sqlx::query_as!(Article,
    r#"INSERT INTO articles
    (user_id, slug, title, description, content, html)
    VALUES ($1, $2, $3, $4, $5, $6)
    RETURNING article_id, user_id, slug, title, description, content, html, views, comment_count, allow_comment, created_at, updated_at"#,
    user_id, 
    slug, 
    article.title, 
    article.description, 
    article.content, html)
    .fetch_one(pool).await?;

    Ok(row)
}
