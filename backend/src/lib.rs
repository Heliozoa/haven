//! Haven

#[macro_use]
extern crate tracing;

pub mod api;
pub mod authorization;
pub mod domain;
pub mod error;
pub mod routes;
pub mod schema;
pub mod static_files;

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Extension,
};
use diesel::{r2d2::ConnectionManager, PgConnection};
use tower_cookies::Cookies;

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Pool = Extension<ConnectionPool>;

pub struct CookiesWrapper(Cookies);

#[axum::async_trait]
impl<S> FromRequestParts<S> for CookiesWrapper
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts
            .extensions
            .get::<Cookies>()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(CookiesWrapper(cookies.clone()))
    }
}
