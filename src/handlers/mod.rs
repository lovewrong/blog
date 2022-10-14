pub mod articles;
pub mod index;
pub mod user;

use axum::http::{header, HeaderMap, HeaderValue};

pub fn set_cookie(key: &str, value: &str) -> HeaderMap {
    let cookie = format!("{}={}; ", key, value);
    let cookie = HeaderValue::from_str(&cookie).expect("Invalid cookie value");
    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie);
    headers
}
