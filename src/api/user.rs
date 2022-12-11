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

#[post("/user")]
async fn create_user(user: web::Json<NewUser>) -> Result<HttpResponse, APIError> {
    let user = User::create(user.into_inner());
    match user {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(e) => {
            log::warn!("Could not create User: {e:?}");
            Err(APIError::InternalServerError)
        }
    }
}

#[get("/user/{id}")]
pub async fn get_user(id: web::Path<i32>) -> Result<HttpResponse, APIError> {
    let user = User::get(*id);
    match user {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(e) => {
            log::warn!("Could not create User: {e:?}");
            Err(APIError::InternalServerError)
        }
    }
}

#[get("/user")]
pub async fn get_all_user() -> Result<HttpResponse, APIError> {
    let all_user = User::get_all();
    match all_user {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(e) => {
            log::warn!("Could not query all Users: {e:?}");
            Err(APIError::InternalServerError)
        }
    }
}

#[delete("/user/{id}")]
pub async fn delete_user(id: web::Path<i32>) -> Result<HttpResponse, APIError> {
    let id = id.into_inner();
    let deleted_user = User::delete(id);
    match deleted_user {
        Ok(1) => Ok(HttpResponse::Ok().json("Deleted User")),
        Ok(0) => {
            log::warn!("There exists no User with the id: {:?}", id);
            Err(APIError::BadUserRequest)
        }
        Ok(n) => {
            log::warn!("Deleted {n:?} Users !");
            Ok(HttpResponse::Ok().json(n))
        }
        Err(e) => {
            log::warn!("Could not delete user: {e:?}");
            Err(APIError::InternalServerError)
        }
    }
}

#[put("/user/{id}")]
async fn update_user(
    id: web::Path<i32>,
    user: web::Json<NewUser>,
) -> Result<HttpResponse, APIError> {
    let user = user.into_inner();
    let update_user = User {
        id: id.into_inner(),
        name: user.name,
        email: user.email,
    };
    let user = User::update(update_user);
    match user {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(e) => {
            log::warn!("Could not update user: {e:?}");
            Err(APIError::InternalServerError)
        }
    }
}
