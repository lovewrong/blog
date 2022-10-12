use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get_service, Router};
use tera::Tera;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::handlers::*;
use crate::AppState;

fn router(state: AppState, tera: Tera) -> Router {
    Router::new()
        .merge(index::router())
        .merge(user::router())
        .merge(articles::router())
        .layer(Extension(Arc::new(state)))
        .layer(Extension(tera))
        .fallback(get_service(ServeDir::new(".")).handle_error(handle_error))
        .layer(TraceLayer::new_for_http())
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "static file not found")
}

pub async fn run(addr: &SocketAddr, state: AppState, tera: Tera) -> anyhow::Result<()> {
    let app = router(state, tera);
    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
