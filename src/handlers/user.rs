use std::sync::Arc;

use axum::extract::{Extension, Form, Path};
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::{get, post, Router};
use axum::{headers, TypedHeader};
use tera::{Context, Tera};

use crate::auth::{AuthUser, OptionalAuthUser, COOKIE_NAME};
use crate::db::user;
use crate::handlers::set_cookie;
use crate::models::users::{CreateUser, LoginUser};
use crate::{AppState, Result};

pub fn router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", get(logout))
        .route("/user/:username", get(user_details))
}

async fn register(
    Form(form): Form<CreateUser>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    let user = user::post_new_user(&app_state.db, form).await?;
    let session: AuthUser = user.into();
    let key = session.storage(&app_state.redis).await?;
    let headers = set_cookie(COOKIE_NAME, &key);

    Ok((headers, Redirect::to("/")))
}

async fn login(
    Form(form): Form<LoginUser>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    let user = user::get_user_by_email(&app_state.db, form).await?;
    let session: AuthUser = user.into();
    let key = session.storage(&app_state.redis).await?;
    let headers = set_cookie(COOKIE_NAME, &key);

    Ok((headers, Redirect::to("/")))
}

async fn logout(
    user: OptionalAuthUser,
    TypedHeader(cookie): TypedHeader<headers::Cookie>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    if let OptionalAuthUser(Some(_user)) = user {
        let key = cookie.get(COOKIE_NAME).expect("Invalid cookie");
        AuthUser::remove(&app_state.redis, key).await?;
    }
    let headers = set_cookie(COOKIE_NAME, "");

    Ok((headers, Redirect::to("/")))
}

async fn user_details(
    user: OptionalAuthUser,
    Path(username): Path<String>,
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(tera): Extension<Tera>,
) -> Result<impl IntoResponse> {
    let mut context = Context::new();
    if let OptionalAuthUser(Some(user)) = user {
        context.insert("current_user", &user);
        if user.username == username {
            context.insert("edit", &username)
        }
    }
    let user = user::get_user_details_by_username(&app_state.db, username).await?;
    context.insert("user", &user);

    Ok(Html(tera.render("user.html", &context)?))
}
