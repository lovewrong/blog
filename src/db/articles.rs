use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::models::articles::{Article, CreateArticle};
use crate::utils::{slugify, markdown_to_html};
use crate::{Error, Result};

pub async fn get_articles_by_page(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Article>> {
    let rows = sqlx::query_as!(Article, "select * from articles order by created_at desc limit $1 offset $2",
                    limit, offset).fetch_all(pool).await?;
    Ok(rows)
}

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
    let html = markdown_to_html(&article.content);

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

pub async fn remove_article(pool: &PgPool, user: AuthUser, article_id: Uuid) -> Result<()> {
    let _row = sqlx::query!(
        "DELETE FROM articles WHERE user_id = $1 and article_id = $2",
        user.user_id,
        article_id).execute(pool).await?;
    Ok(())
}