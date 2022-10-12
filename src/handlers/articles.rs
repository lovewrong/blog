use std::sync::Arc;

use axum::extract::{Extension, Form, Path};
use axum::response::Html;
use axum::routing::{get, Router};
use tera::{Context, Tera};

use crate::db::articles;
use crate::models::articles::CreateArticle;
use crate::{AppState, Result};

pub fn router() -> Router {
    Router::new()
        .route("/article", get(new_article).post(post_new_article))
        .route("/article/:sulg", get(get_article_details))
        .route("/articles", get(get_articles))
}

async fn get_article_details(
    Path(slug): Path<String>,
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(tera): Extension<Tera>,
) -> Result<Html<String>> {
    let article = articles::get_article_details_for_db(&app_state.db, &slug).await?;

    let mut context = Context::new();
    context.insert("article", &article);

    Ok(Html(tera.render("article.html", &context)?))
}

async fn new_article(Extension(tera): Extension<Tera>) -> Result<Html<String>> {
    Ok(Html(tera.render("new_article.html", &Context::new())?))
}

async fn post_new_article(
    Form(article): Form<CreateArticle>,
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(tera): Extension<Tera>,
) -> Result<Html<String>> {
    let article = articles::post_new_article_db(&app_state.db, article).await?;
    let mut context = Context::new();
    context.insert("article", &article);

    Ok(Html(tera.render("article.html", &context)?))
}

async fn get_articles(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(tera): Extension<Tera>,
) -> Result<Html<String>> {
    let articles = articles::get_all_article_for_db(&app_state.db).await?;

    let mut context = Context::new();
    context.insert("articles", &articles);

    Ok(Html(tera.render("articles.html", &context)?))
}
