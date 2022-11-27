//! Contains types and functions for authorizing frontend users.

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{extract::FromRequestParts, http::request::Parts};
use hyper::StatusCode;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookies, Key};

#[derive(Debug, Deserialize, Serialize)]
pub struct Authentication {
    pub user_id: i32,
}

impl Authentication {
    /// __Host- cookie: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie#attributes
    pub const COOKIE_NAME: &'static str = "__Host-auth";
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Authentication
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let extensions = &parts.extensions;
        let cookies = extensions.get::<Cookies>().unwrap();
        let key = extensions.get::<Key>().unwrap();
        let private = cookies.private(key);
        if let Some(authentication) = private
            .get(Self::COOKIE_NAME)
            .and_then(|c| serde_json::from_str(c.value()).ok())
        {
            Ok(authentication)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

pub fn hash_password(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password, salt.as_ref())
        .expect("Failed to hash password");
    hash.to_string()
}

pub fn verify_password(password: &[u8], hash: &str) -> bool {
    let argon2 = Argon2::default();
    let hash = PasswordHash::new(hash).expect("Invalid hash");
    argon2.verify_password(password, &hash).is_ok()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn password_hash_round_trip() {
        let pass = "2442ed22d8f601b5053f5f0c81609bb4";
        let hash = hash_password(pass.as_bytes());
        assert!(verify_password(pass.as_bytes(), &hash));
    }
}
