use crate::model::user::{NewUser, User};
use actix_web::{
    delete,
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put, web, HttpResponse,
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
    let user = User::create(user.into_inner());
    Ok(HttpResponse::Ok().json(user))
}

#[get("/user/{id}")]
pub async fn get_user(id: web::Path<i32>) -> Result<HttpResponse, UserError> {
    User::get(*id);
    Ok(HttpResponse::Ok().json("Its fine"))
}

#[get("/user")]
pub async fn get_all_user() -> Result<HttpResponse, UserError> {
    User::get_all();
    Ok(HttpResponse::Ok().json("All User"))
}

#[delete("/user/{id}")]
pub async fn delete_user(id: web::Path<i32>) -> Result<HttpResponse, UserError> {
    User::delete(id.into_inner());
    Ok(HttpResponse::Ok().json("User deleted!"))
}

#[put("/user/{id}")]
async fn update_user(
    id: web::Path<i32>,
    user: web::Json<NewUser>,
) -> Result<HttpResponse, UserError> {
    let user = user.into_inner();
    let id = id.into_inner();
    let update_user = User {
        id,
        name: user.name,
        email: user.email,
    };
    User::update(update_user);
    Ok(HttpResponse::Ok().json("Updated User"))
}
