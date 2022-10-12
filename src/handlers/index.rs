use std::sync::Arc;

use axum::extract::Extension;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, Router};
use tera::{Context, Tera};
use tracing::info;

use crate::extractor::MaybeAuthUser;
use crate::{AppState, Result};

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login))
}

async fn index(
    user: MaybeAuthUser,
    Extension(tera): Extension<Tera>,
    Extension(_app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    info!("index");
    println!("user: {:?}", user);
    let mut context = Context::new();
    if let MaybeAuthUser(Some(user)) = user {
        context.insert("user", &user);
    }
    Ok(Html(tera.render("index.html", &context)?))
}

async fn login(Extension(tera): Extension<Tera>) -> Result<impl IntoResponse> {
    Ok(Html(tera.render("login.html", &Context::new())?))
}
