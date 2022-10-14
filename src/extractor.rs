use std::sync::Arc;

use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    headers, TypedHeader,
};

use crate::{
    auth::{AuthUser, OptionalAuthUser, COOKIE_NAME},
    AppState, Error, Result,
};

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
            req.extract().await.expect("cookie extraction failed");
        let cookies = cookies.ok_or(Error::Unauthorized)?;
        let key = cookies.get(COOKIE_NAME).ok_or(Error::Unauthorized)?;

        Self::from_authorization(&state.redis, key).await
    }
}

#[async_trait]
impl<B> FromRequest<B> for OptionalAuthUser
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

        match session_cookie {
            Some(session) => match AuthUser::from_authorization(&state.redis, session).await {
                Ok(user) => Ok(Self(Some(user))),
                Err(_) => Ok(Self(None)),
            },
            None => Ok(Self(None)),
        }
    }
}
