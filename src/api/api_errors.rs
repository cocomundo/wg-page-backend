use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use derive_more::Display;

#[derive(Debug, Display)]
pub enum APIError {
    UserNotFound,
    UserUpdateFailure,
    UserCreationFailure,
    BadUserRequest,
    InternalServerError,
}

impl ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            APIError::UserNotFound => StatusCode::NOT_FOUND,
            APIError::UserUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            APIError::UserCreationFailure => StatusCode::FAILED_DEPENDENCY,
            APIError::BadUserRequest => StatusCode::BAD_REQUEST,
            APIError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
