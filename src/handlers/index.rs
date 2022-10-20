use std::sync::Arc;

use axum::extract::Extension;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, Router};
use tera::{Context, Tera};

use crate::auth::OptionalAuthUser;
use crate::{AppState, Result};

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login))
}

async fn index(
    auth: OptionalAuthUser,
    Extension(tera): Extension<Tera>,
    Extension(_app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    let mut context = Context::new();

    if let OptionalAuthUser(Some(user)) = auth {
        context.insert("current_user", &user);
    }
    Ok(Html(tera.render("index.html", &context)?))
}

async fn login(Extension(tera): Extension<Tera>) -> Result<impl IntoResponse> {
    Ok(Html(tera.render("login.html", &Context::new())?))
}
