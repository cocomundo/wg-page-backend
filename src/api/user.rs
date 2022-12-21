use crate::{
    api::api_errors::APIError,
    model::user::{User, UserInput},
};
use actix_web::{delete, get, post, put, web, HttpResponse};

#[post("/user")]
async fn create_user(user: web::Json<UserInput>) -> Result<HttpResponse, APIError> {
    let user = User::create(user.into_inner());
    match user {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(e) => {
            log::warn!("Could not create User: {e:?}");
            Err(APIError::InternalServerError)
        }
    }
}

#[get("/user/{email}")]
pub async fn get_user(email: web::Path<String>) -> Result<HttpResponse, APIError> {
    let email = email.into_inner();
    let user = User::get(&email);
    match user {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(e) => {
            log::warn!("Could not create User: {e:?}");
            Err(APIError::InternalServerError)
        }
    }
}

#[get("/user")]
pub async fn get_all_users() -> Result<HttpResponse, APIError> {
    let all_user = User::get_all();
    match all_user {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(e) => {
            log::warn!("Could not query all Users: {e:?}");
            Err(APIError::InternalServerError)
        }
    }
}

#[delete("/user/{email}")]
pub async fn delete_user(email: web::Path<String>) -> Result<HttpResponse, APIError> {
    let email = email.into_inner();
    let deleted_user = User::delete(&email);
    match deleted_user {
        Ok(1) => Ok(HttpResponse::Ok().json("Deleted User")),
        Ok(0) => {
            log::warn!("There user {:?} does not exist", email);
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

#[put("/user/{email}")]
async fn update_user(user: web::Json<UserInput>) -> Result<HttpResponse, APIError> {
    let user = user.into_inner();
    let update_user = User {
        name: user.name,
        email: user.email,
        pwhash: user.pwhash, // TODO: make hash from pw
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
