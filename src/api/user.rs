use crate::{
    api::api_errors::APIError,
    model::user::{NewUser, User},
};
use actix_web::{delete, get, post, put, web, HttpResponse};

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
