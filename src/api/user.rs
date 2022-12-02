use crate::model::user::User;
use actix_web::{
    get,
    post,
    put,
    delete,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{Serialize, Deserialize};
use derive_more::{Display};

#[derive(Debug, Display)]
pub enum UserError {
    UserNotFound
    UserUpdateFailure
    UserCreationFailure
    BadUserRequest
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserError::UserNotFound => StatusCode::NOT_FOUND,
            UserError::UserUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            UserError::UserCreationFailure => StatusCode::FAILED_DEPENDENCY,
            UserError::BadUserRequest => StatusCode::BAD_REQUEST
        }
    }
}

#[post("/user")]
async fn create(user: web::Json<NewUser>) -> Result<HttpResponse, UserError> {
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}
#[get("/user/{id}")]
pub async fn get_user(id: web::Path<i32>) -> Result<HttpResponse, UserError> {
    let user = User::get(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/user/id")]
pub async fn delete_user(id: web::Path<i32>) -> Result<HttpResponse, UserError> {
    let deleted_user = Employees::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_user })))
}
