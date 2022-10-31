pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod extractor;
pub mod handlers;
pub mod models;
pub mod serve;
pub mod utils;

use redis::Client;
use sqlx::postgres::PgPool;

pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct AppState {
    pub db: PgPool,
    pub redis: Client,
}
