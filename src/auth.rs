use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use uuid::Uuid;

use crate::models::users::User;
use crate::utils::generate_string;
use crate::{Error, Result};

pub const COOKIE_NAME: &'static str = "session";
pub const COOKIE_LIFETIME: usize = 7 * 24 * 60 * 60;

#[derive(Debug)]
pub struct OptionalAuthUser(pub Option<AuthUser>);

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub username: String,
    pub avatar: Option<String>,
    pub groups: UserGroups,
    pub exp: usize,
}

impl AuthUser {
    pub async fn storage(&self, client: &redis::Client) -> Result<String> {
        let key = generate_string();
        let mut conn = client.get_async_connection().await?;
        conn.set_ex(&key, json!(self).to_string(), self.exp).await?;
        Ok(key)
    }

    pub async fn from_authorization(client: &redis::Client, key: &str) -> Result<Self> {
        let mut conn = client.get_async_connection().await?;
        let value: String = conn.get(key).await.map_err(|_| Error::Unauthorized)?;
        Ok(from_str(&value).expect("Deserialize failed"))
    }

    pub async fn remove(client: &redis::Client, key: &str) -> Result<()> {
        let mut conn = client.get_async_connection().await?;
        conn.del(key).await?;
        Ok(())
    }
}

impl From<User> for AuthUser {
    fn from(user: User) -> Self {
        Self {
            user_id: user.user_id,
            username: user.username,
            avatar: user.url,
            groups: UserGroups::new(user.groups).expect("invalid user groups"),
            exp: COOKIE_LIFETIME,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum UserGroups {
    Administrator,
    Auditor,
    Contributor,
}

impl UserGroups {
    pub fn new(level: i16) -> anyhow::Result<Self> {
        match level {
            0 => Ok(UserGroups::Administrator),
            1 => Ok(UserGroups::Auditor),
            2 => Ok(UserGroups::Contributor),
            _ => Err(anyhow::anyhow!("Invalid level: {}", level)),
        }
    }
}
