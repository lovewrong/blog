use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::models::comments::{Comment, CreateComment};
use crate::{Error, Result};

pub async fn get_comments_by_article_id(pool: &PgPool, article_id: Uuid) -> Result<Vec<Comment>> {
    let rows = sqlx::query_as!(
        Comment,
        "SELECT * FROM comments WHERE article_id = $1 ORDER BY created_at",
        article_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn post_new_comment(
    pool: &PgPool,
    article_id: Uuid,
    user_id: Uuid,
    comment: CreateComment,
) -> Result<()> {
    let _row = sqlx::query!(
        "INSERT INTO comments 
    (article_id, user_id, content)
    VALUES
    ($1, $2, $3)",
        article_id,
        user_id,
        comment.content
    )
    .execute(pool)
    .await?;

    Ok(())
}
