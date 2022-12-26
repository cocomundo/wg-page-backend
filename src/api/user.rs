use crate::{
    api::api_errors::APIError,
    model::user::{User, UserInput},
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Serialize, Deserialize};

use super::utils::create_hash;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub email: String,
    pub name: String,
    pub pw: String,
}

#[post("/user")]
async fn create_user(user: web::Json<UserData>) -> Result<HttpResponse, APIError> {
    let user_data = user.into_inner();
    let pwhash = create_hash(&user_data.pw.into_bytes()).unwrap();
    let user = UserInput{
        email : user_data.email,
        name : user_data.name,
        pwhash,
    };

    let user = User::create(user);
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
async fn update_user(current_email: web::Path<String>, user_data: web::Json<UserData>) -> Result<HttpResponse, APIError> {
    let user_data = user_data.into_inner();
    let pwhash = create_hash(&user_data.pw.into_bytes()).unwrap();
    let user = User{
        email : user_data.email,
        name : user_data.name,
        pwhash,
    };
    let updated_user = User::update(&current_email.into_inner(), user);
    match updated_user {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(e) => {
            log::warn!("Could not update user: {e:?}");
            Err(APIError::InternalServerError)
        }
    }
}
