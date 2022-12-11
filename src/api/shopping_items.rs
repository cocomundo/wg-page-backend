use crate::{
    api::api_errors::APIError,
    model::shopping_items::{NewShoppingItem, ShoppingItem},
};
use actix_web::{delete, get, post, put, web, HttpResponse};

#[post("/shopping_item")]
async fn create_shopping_item(shopping_item: web::Json<NewShoppingItem>) -> Result<HttpResponse, APIError> {
    let item = ShoppingItem::create(shopping_item.into_inner());
    match item {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(e) => {
            log::warn!("Could not create Shopping Item: {e:?}");
            Err(APIError::InternalServerError)
        }
    }
}

#[get("/shopping_item/{id}")]
pub async fn get_shopping_item(id: web::Path<i32>) -> Result<HttpResponse, APIError> {
    let item = ShoppingItem::get(*id);
    match item {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(e) => {
            log::warn!("Could not create Shopping Item: {e:?}");
            Err(APIError::InternalServerError)
        }
    }
}

#[get("/shopping_item")]
pub async fn get_all_shopping_items() -> Result<HttpResponse, APIError> {
    let all_items = ShoppingItem::get_all();
    match all_items {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(e) => {
            log::warn!("Could not query all Shopping Items: {e:?}");
            Err(APIError::InternalServerError)
        }
    }
}

#[delete("/shopping_item/{id}")]
pub async fn delete_shopping_item(id: web::Path<i32>) -> Result<HttpResponse, APIError> {
    let id = id.into_inner();
    let deleted_item = ShoppingItem::delete(id);
    match deleted_item {
        Ok(1) => Ok(HttpResponse::Ok().json("Deleted Shopping Item")),
        Ok(0) => {
            log::warn!("There exists no Shopping Item with the id: {:?}", id);
            Err(APIError::BadUserRequest)
        }
        Ok(n) => {
            log::warn!("Deleted {n:?} Shopping Items !");
            Ok(HttpResponse::Ok().json(n))
        }
        Err(e) => {
            log::warn!("Could not delete Shopping Item: {e:?}");
            Err(APIError::InternalServerError)
        }
    }
}

#[put("/shopping_item/{id}")]
async fn update_shopping_item(
    id: web::Path<i32>,
    item: web::Json<NewShoppingItem>,
) -> Result<HttpResponse, APIError> {
    let item = item.into_inner();
    let update_item = ShoppingItem {
        id: id.into_inner(),
        name: item.name,
        quantity: item.quantity,
    };
    let item = ShoppingItem::update(update_item);
    match item {
        Ok(u) => Ok(HttpResponse::Ok().json(u)),
        Err(e) => {
            log::warn!("Could not update Item: {e:?}");
            Err(APIError::InternalServerError)
        }
    }
}

