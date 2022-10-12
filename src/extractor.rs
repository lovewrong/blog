use std::sync::Arc;

use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    headers, TypedHeader,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use tracing::info;
use uuid::Uuid;

use crate::{AppState, Error, Result};

pub const COOKIE_NAME: &'static str = "session";

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthUser {
    pub user_id: Uuid,
    exp: usize,
}

#[derive(Debug)]
pub struct MaybeAuthUser(pub Option<AuthUser>);

impl AuthUser {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            exp: 14 * 24 * 60 * 60,
        }
    }

    pub async fn to_redis(&self, client: &redis::Client) -> Result<()> {
        let mut conn = client.get_async_connection().await?;
        conn.set_ex(self.user_id.to_string(), json!(self).to_string(), self.exp)
            .await?;
        Ok(())
    }

    pub async fn from_authorization(client: &redis::Client, key: &str) -> Result<Self> {
        let mut conn = client.get_async_connection().await?;
        info!("from_authorization: {:?}", key);
        let value: String = conn.get(key).await.map_err(|_| Error::Unauthorized)?;
        Ok(from_str(&value).expect("Deserialize failed"))
    }

    pub async fn remove(&self, client: &redis::Client) -> Result<()> {
        let mut conn = client.get_async_connection().await?;
        conn.del(&self.user_id.to_string()).await?;
        Ok(())
    }
}

#[async_trait]
impl<B> FromRequest<B> for AuthUser
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
        let Extension(state): Extension<Arc<AppState>> = Extension::from_request(req)
            .await
            .expect("AppState was not added as an extension.");

        let cookies: Option<TypedHeader<headers::Cookie>> =
            req.extract().await.expect("cookie extraction failed.");
        let session_cookie = cookies.as_ref().and_then(|cookie| cookie.get(COOKIE_NAME));
        println!("Session cookie: {:?}", session_cookie);
        match session_cookie {
            Some(session) => Self::from_authorization(&state.redis, session).await,
            None => Err(Error::Unauthorized),
        }
    }
}

#[async_trait]
impl<B> FromRequest<B> for MaybeAuthUser
where
    B: Send,
{
    type Rejection = Error;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
        let Extension(state): Extension<Arc<AppState>> = Extension::from_request(req)
            .await
            .expect("AppState was not added as an extension.");

        let cookies: Option<TypedHeader<headers::Cookie>> =
            req.extract().await.expect("cookie extraction failed");

        let session_cookie = cookies.as_ref().and_then(|cookie| cookie.get(COOKIE_NAME));
        println!("Session cookie: {:?}", session_cookie);
        match session_cookie {
            Some(session) => match AuthUser::from_authorization(&state.redis, session).await {
                Ok(user) => {
                    println!("logined user: {:?}", user);
                    Ok(Self(Some(user)))
                }
                Err(_) => Ok(Self(None)),
            },
            None => Ok(Self(None)),
        }
    }
}
