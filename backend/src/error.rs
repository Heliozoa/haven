//! Haven error type.

use axum::{
    body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use diesel::result::Error as DieselError;

pub type HavenResult<T> = Result<T, HavenError>;

#[derive(Debug, thiserror::Error)]
pub enum HavenError {
    #[error("Failed to authorize user")]
    Auth,
    #[error("Invalid file upload")]
    InvalidFileUpload,
    #[error("Bad request: {0}")]
    BadRequest(&'static str),

    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
    #[error(transparent)]
    Image(#[from] image::ImageError),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    Diesel(#[from] DieselError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    R2d2(#[from] r2d2::Error),
    #[error(transparent)]
    Multipart(#[from] axum::extract::multipart::MultipartError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

impl IntoResponse for HavenError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            Self::Auth => (StatusCode::FORBIDDEN, body::boxed(body::Empty::new())),
            Self::BadRequest(error) => (
                StatusCode::BAD_REQUEST,
                body::boxed(body::Full::new(error.into())),
            ),
            Self::Diesel(DieselError::NotFound) => {
                (StatusCode::NOT_FOUND, body::boxed(body::Empty::new()))
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                body::boxed(body::Empty::new()),
            ),
        };
        Response::builder().status(status).body(body).unwrap()
    }
}
