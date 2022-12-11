use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct UserError {
    pub error_status_code: u16,
    pub error_message: String,
}

impl UserError {
    pub fn new(error_status_code: u16, error_message: String) -> UserError {
        UserError {
            error_status_code,
            error_message,
        }
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}

impl From<DieselError> for UserError {
    fn from(error: DieselError) -> UserError {
        match error {
            DieselError::DatabaseError(_, err) => UserError::new(409, err.message().to_string()),
            DieselError::NotFound => UserError::new(404, "The user record not found".to_string()),
            err => UserError::new(500, format!("Unknown Diesel error: {}", err)),
        }
    }
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.error_status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = match status_code.as_u16() < 500 {
            true => self.error_message.clone(),
            false => "Internal server error".to_string(),
        };

        HttpResponse::build(status_code).json(json!({ "message": error_message }))
    }
}
