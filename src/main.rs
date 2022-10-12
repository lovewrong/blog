use sqlx::postgres::PgPoolOptions;
use tera::Tera;

use blog::config::Config;
use blog::server;
use blog::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    let db = PgPoolOptions::new().connect(&config.database_url).await?;
    sqlx::migrate!().run(&db).await?;
    let redis = redis::Client::open(config.redis_url)?;
    let tera = Tera::new("templates/*.html")?;
    let app_state = AppState { db, redis };

    tracing::info!("listening on {}", &config.web_address);
    server::run(&config.web_address, app_state, tera).await?;
    Ok(())
}
