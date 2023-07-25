use actix_web::{http::StatusCode, ResponseError};

#[derive(thiserror::Error, Debug)]
pub enum APIError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        match self {
            APIError::UnexpectedError(_) => StatusCode::BAD_REQUEST,
            APIError::ValidationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
