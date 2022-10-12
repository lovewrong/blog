use std::sync::Arc;

use axum::extract::{Extension, Form};
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post, Router};

use crate::db::user;
use crate::extractor::{AuthUser, MaybeAuthUser, COOKIE_NAME};
use crate::handlers::set_cookie;
use crate::models::users::{CreateUser, LoginUser};
use crate::{AppState, Result};

pub fn router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", get(logout))
}

async fn register(
    Form(form): Form<CreateUser>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    let user = user::post_new_user(&app_state.db, form).await?;

    let auth_user = AuthUser::new(user.user_id);
    auth_user.to_redis(&app_state.redis).await?;
    let headers = set_cookie(COOKIE_NAME, &auth_user.user_id.to_string());

    Ok((headers, Redirect::to("/")))
}

async fn login(
    Form(form): Form<LoginUser>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    let user = user::get_user_by_email(&app_state.db, form).await?;
    let auth_user = AuthUser::new(user.user_id);
    auth_user.to_redis(&app_state.redis).await?;
    let headers = set_cookie(COOKIE_NAME, &auth_user.user_id.to_string());
    Ok((headers, Redirect::to("/")))
}

async fn logout(
    user: MaybeAuthUser,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    if let MaybeAuthUser(Some(user)) = user {
        user.remove(&app_state.redis).await?;
    }
    let headers = set_cookie(COOKIE_NAME, "");
    Ok((headers, Redirect::to("/")))
}
