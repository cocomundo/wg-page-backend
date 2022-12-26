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

    let user = User::create(user)?;
    Ok(HttpResponse::Ok().json(user))
    // match user {
    //     Ok(u) => Ok(HttpResponse::Ok().json(u)),
    //     Err(e) => {
    //         log::warn!("Could not create User: {e:?}");
    //         Err(APIError::InternalServerError)
    //     }
    // }
}

#[get("/user/{email}")]
pub async fn get_user(email: web::Path<String>) -> Result<HttpResponse, APIError> {
    let email = email.into_inner();
    let user = User::get(&email)?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/user")]
pub async fn get_all_users() -> Result<HttpResponse, APIError> {
    let all_users = User::get_all()?;
    Ok(HttpResponse::Ok().json(all_users))
}

#[delete("/user/{email}")]
pub async fn delete_user(email: web::Path<String>) -> Result<HttpResponse, APIError> {
    let email = email.into_inner();
    let deleted_user = User::delete(&email)?;
    Ok(HttpResponse::Ok().json(deleted_user))
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
    let updated_user = User::update(&current_email.into_inner(), user)?;
    Ok(HttpResponse::Ok().json(updated_user))
}
