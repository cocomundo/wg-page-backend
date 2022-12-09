use crate::model::user::{self, NewUser};
use actix_web::{
    delete,
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put, web,
    HttpResponse,
};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum UserError {
    UserNotFound,
    UserUpdateFailure,
    UserCreationFailure,
    BadUserRequest,
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
            UserError::BadUserRequest => StatusCode::BAD_REQUEST,
        }
    }
}

#[post("/user")]
async fn create_user(user: web::Json<NewUser>) -> Result<HttpResponse, UserError> {
    let user = user.into_inner();
    user::create_user(&user.name, &user.email);
    Ok(HttpResponse::Ok().json(user))
}

#[get("/user/{id}")]
pub async fn get_user(id: web::Path<i32>) -> Result<HttpResponse, UserError> {
    user::show_users();
    Ok(HttpResponse::Ok().json("Its fine"))
}

#[delete("/user/{id}")]
pub async fn delete_user(id: web::Path<i32>) -> Result<HttpResponse, UserError> {
    user::delete_user(id.into_inner());
    Ok(HttpResponse::Ok().json("User deleted!"))
}
