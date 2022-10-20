use std::sync::Arc;

use axum::extract::{Extension, Form, Path};
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::{get, Router};
use tera::{Context, Tera};
use uuid::Uuid;

use crate::auth::{AuthUser, OptionalAuthUser};
use crate::db::articles;
use crate::models::articles::CreateArticle;
use crate::{AppState, Result};

pub fn router() -> Router {
    Router::new()
        .route("/article", get(new_article).post(post_new_article))
        .route("/article/:sulg", get(get_article_details))
        .route("/articles", get(get_articles))
        .route("/article/remove/:id", get(remove_article))
}

async fn get_article_details(
    auth: OptionalAuthUser,
    Path(slug): Path<String>,
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(tera): Extension<Tera>,
) -> Result<Html<String>> {
    let article = articles::get_article_details_for_db(&app_state.db, &slug).await?;

    let mut context = Context::new();
    if let OptionalAuthUser(Some(user)) = auth {
        context.insert("current_user", &user);
    }
    context.insert("article", &article);

    Ok(Html(tera.render("article.html", &context)?))
}

async fn new_article(
    _user: AuthUser,
    Extension(tera): Extension<Tera>,
) -> Result<impl IntoResponse> {
    Ok(Html(tera.render("new_article.html", &Context::new())?))
}

async fn post_new_article(
    user: AuthUser,
    Form(article): Form<CreateArticle>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    let article = articles::post_new_article_db(&app_state.db, user, article).await?;
    let article_url = format!("/article/{}", article.slug);
    Ok(Redirect::to(&article_url))
}

async fn get_articles(
    auth: OptionalAuthUser,
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(tera): Extension<Tera>,
) -> Result<impl IntoResponse> {
    let articles = articles::get_all_article_for_db(&app_state.db).await?;

    let mut context = Context::new();
    if let OptionalAuthUser(Some(user)) = auth {
        context.insert("current_user", &user);
    }
    context.insert("articles", &articles);

    Ok(Html(tera.render("articles.html", &context)?))
}

async fn remove_article(auth: AuthUser, Path(id): Path<Uuid>, Extension(app_state): Extension<Arc<AppState>>) -> Result<impl IntoResponse> {
    articles::remove_article(&app_state.db, auth, id).await?;
    Ok(Redirect::to("/articles"))
    // TODO: redirect to article page
}