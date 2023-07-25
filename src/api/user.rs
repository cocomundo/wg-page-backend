use crate::{
    api::api_errors::APIError,
    model::user::{User, UserData},
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};

use super::utils::create_hash;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInput {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserOutput {
    pub email: String,
    pub name: String,
}

impl UserOutput {
    fn from_user(user: &User) -> Self {
        UserOutput {
            email: user.email.clone(),
            name: user.name.clone(),
        }
    }
}

#[post("/user")]
async fn create_user(user_input: web::Json<UserInput>) -> Result<HttpResponse, APIError> {
    let user_input = user_input.into_inner();
    let pwhash = create_hash(&user_input.password.into_bytes()).unwrap();
    let user = UserData {
        email: user_input.email,
        name: user_input.name,
        pwhash,
    };

    let user_output = UserOutput::from_user(&User::create(user)?);
    Ok(HttpResponse::Ok().json(user_output))
}

#[get("/user/{email}")]
pub async fn get_user(email: web::Path<String>) -> Result<HttpResponse, APIError> {
    let email = email.into_inner();
    let user_output = UserOutput::from_user(&User::get(&email)?);
    Ok(HttpResponse::Ok().json(user_output))
}

#[get("/user")]
pub async fn get_all_users() -> Result<HttpResponse, APIError> {
    let all_users = User::get_all()?;
    let mut all_users_output: Vec<UserOutput> = Vec::new();
    for user in all_users.iter() {
        all_users_output.push(UserOutput::from_user(user));
    }
    Ok(HttpResponse::Ok().json(all_users_output))
}

#[delete("/user/{email}")]
pub async fn delete_user(email: web::Path<String>) -> Result<HttpResponse, APIError> {
    let email = email.into_inner();
    User::delete(&email)?;
    Ok(HttpResponse::Ok().finish())
}

#[put("/user/{email}")]
async fn update_user(
    current_email: web::Path<String>,
    user_input: web::Json<UserInput>,
) -> Result<HttpResponse, APIError> {
    let user_input = user_input.into_inner();
    let pwhash = create_hash(&user_input.password.into_bytes()).unwrap();
    let user = User {
        email: user_input.email,
        name: user_input.name,
        pwhash,
    };
    let updated_user_data =
        UserOutput::from_user(&User::update(&current_email.into_inner(), user)?);
    Ok(HttpResponse::Ok().json(updated_user_data))
}
