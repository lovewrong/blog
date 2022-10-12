use std::sync::Arc;

use axum::extract::{Extension, Form};
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post, Router};
use tracing::info;
// use tera::{Context, Tera};

use crate::db::user;
use crate::extractor::{AuthUser, COOKIE_NAME};
use crate::handlers::set_cookie;
use crate::models::users::{CreateUser, LoginUser};
use crate::{AppState, Result};

pub fn router() -> Router {
    Router::new()
        .route("/user/register_action", post(register_action))
        .route("/user/login_action", post(login_action))
        .route("/user/logout_action", get(logout_action))
}

async fn register_action(
    Form(form): Form<CreateUser>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    let user = user::post_new_user(&app_state.db, form).await?;

    let auth_user = AuthUser::new(user.user_id);
    auth_user.to_redis(&app_state.redis).await?;
    let headers = set_cookie(COOKIE_NAME, &auth_user.user_id.to_string());

    Ok((headers, Redirect::to("/")))
}

async fn login_action(
    Form(form): Form<LoginUser>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    let user = user::get_user_by_email(&app_state.db, form).await?;

    let auth_user = AuthUser::new(user.user_id);
    info!("{:?}", auth_user);
    auth_user.to_redis(&app_state.redis).await?;
    let headers = set_cookie(COOKIE_NAME, &auth_user.user_id.to_string());
    println!("new headers: {:?}", headers);
    Ok((headers, Redirect::to("/")))
}

async fn logout_action(
    user: AuthUser,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    user.remove(&app_state.redis).await?;
    let headers = set_cookie(COOKIE_NAME, "");
    Ok((headers, Redirect::to("/")))
}
