use std::env;
use std::net::SocketAddr;

use dotenvy::dotenv;

pub struct Config {
    pub web_address: SocketAddr,
    pub database_url: String,
    pub redis_url: String,
}

impl Config {
    pub fn from_env() -> Config {
        dotenv().ok();

        let web_address = env::var("WEB_ADDRESS").expect("web address is not set!");
        let web_address: SocketAddr = web_address.parse().expect("web address parse failed!");
        let database_url = env::var("DATABASE_URL").expect("database url is not set!");
        let redis_url = env::var("REDIS_URL").expect("redis url is not set!");

        Self {
            web_address,
            database_url,
            redis_url,
        }
    }
}
