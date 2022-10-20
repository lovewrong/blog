use sqlx::postgres::PgPool;

use crate::models::users::{CreateUser, LoginUser, User};
use crate::utils::{hash_password, verify_password};
use crate::{Error, Result};

pub async fn post_new_user(pool: &PgPool, user: CreateUser) -> Result<User> {
    let password_hash = hash_password(user.password).await?;

    let row = sqlx::query_as!(User,
    r#"INSERT INTO users
    (username, email, password_hash)
    VALUES ($1, $2, $3)
    RETURNING user_id, username, email, bio, url, password_hash, groups, disabled, created_at, updated_at"#, 
    &user.username,
    &user.email,
    &password_hash).fetch_one(pool).await?;

    Ok(row)
}

pub async fn get_user_by_email(pool: &PgPool, user: LoginUser) -> Result<User> {
    let row = sqlx::query_as!(User, r#"select * from users where email = $1"#, user.email)
        .fetch_optional(pool)
        .await?
        .ok_or(Error::unprocessable_entity([("email", "does not exist")]))?;
    verify_password(user.password, row.password_hash.clone()).await?;
    Ok(row)
}

pub async fn get_user_details_by_username(pool: &PgPool, username: String) -> Result<User> {
    let row = sqlx::query_as!(User, r#"select * from users where username = $1"#, username)
        .fetch_optional(pool)
        .await?
        .ok_or(Error::NotFound)?;

    Ok(row)
}
