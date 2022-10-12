use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub bio: String,
    pub url: Option<String>,
    pub password_hash: String,
    pub groups: i16,
    pub disabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub enum UserGroups {
    Admin,
    User,
    Visitor,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct UserOptions {
    pub user_option_id: Uuid,
    pub user_id: Uuid,
    pub option_name: String,
    pub option_value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

