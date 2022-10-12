use anyhow::Context;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use itertools::Itertools;

use crate::{Error, Result};

/// Convert a title string to a slug for identifying an article.
///
/// E.g. `slugify("Doctests are the Bee's Knees") == "doctests-are-the-bees-knees"`
///
// (Sadly, doctests are not run on private functions it seems.)
pub fn slugify(string: &str) -> String {
    const QUOTE_CHARS: &[char] = &['\'', '"'];

    string
        // Split on anything that isn't a word character or quotation mark.
        // This has the effect of keeping contractions and possessives together.
        .split(|c: char| !(QUOTE_CHARS.contains(&c) || c.is_alphanumeric()))
        // If multiple non-word characters follow each other then we'll get empty substrings
        // so we'll filter those out.
        .filter(|s| !s.is_empty())
        .map(|s| {
            // Remove quotes from the substring.
            //
            // This allocation is probably avoidable with some more iterator hackery but
            // at that point we'd be micro-optimizing. This function isn't called all that often.
            let mut s = s.replace(QUOTE_CHARS, "");
            // Make the substring lowercase (in-place operation)
            s.make_ascii_lowercase();
            s
        })
        .join("-")
}

pub fn markdown_to_html(_markdown: &str) -> String {
    todo!()
}

// pub fn hash(pwd: &str) -> Result<String> {
//     let salt = SaltString::generate(&mut OsRng);
//     let argon2 = Argon2::default();
//     Ok(argon2.hash_password(pwd.as_bytes(), &salt)?.to_string())
// }

// pub fn verify(pwd: &str, hashed_pwd: &str) -> Result<()> {
//     tracing::info!("verify start");
//     let pwd_hash = PasswordHash::new(hashed_pwd)?;
//     tracing::info!("verify step 1");
//     let argon2 = Argon2::default();
//     tracing::info!("verify step 2");
//     let r = argon2.verify_password(pwd.as_bytes(), &pwd_hash)?;
//     tracing::info!("verify end");
//     Ok(r)
// }

pub async fn hash_password(password: String) -> Result<String> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Ok(argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
            .to_string())
    })
    .await
    .context("panic in generating password hash")??)
}

pub async fn verify_password(password: String, password_hash: String) -> Result<()> {
    Ok(tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;
        let argon2 = Argon2::default();
        argon2
            .verify_password(&password.as_bytes(), &hash)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash")??)
}