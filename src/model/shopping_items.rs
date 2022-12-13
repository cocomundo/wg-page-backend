use crate::db::{
    establish_connection,
    schema::shopping_items::{self, dsl::*},
};
use anyhow::{Context, Error};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = shopping_items)]
pub struct NewShoppingItem {
    pub name: String,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug, AsChangeset, Eq, PartialEq)]
pub struct ShoppingItem {
    pub id: i32,
    pub name: String,
    pub quantity: i32,
}

impl ShoppingItem {
    pub fn create(new_item: NewShoppingItem) -> Result<Self, Error> {
        let mut connection = establish_connection()?;
        diesel::insert_into(shopping_items)
            .values(&new_item)
            .get_result(&mut connection)
            .with_context(|| format!("Could not create new user: {new_item:?}"))
    }

    pub fn update(user: ShoppingItem) -> Result<Self, Error> {
        let mut connection = establish_connection()?;

        diesel::update(shopping_items.find(user.id))
            .set(&user)
            .get_result(&mut connection)
            .with_context(|| format!("Could not update user with id: {:?}", user.id))
    }

    pub fn delete(i: i32) -> Result<usize, Error> {
        let mut connection = establish_connection()?;
        diesel::delete(shopping_items.find(i))
            .execute(&mut connection)
            .with_context(|| format!("Could not delete user with id: {i}"))
    }

    pub fn get(i: i32) -> Result<Self, Error> {
        let mut connection = establish_connection()?;
        shopping_items
            .filter(shopping_items::id.eq(i))
            .first(&mut connection)
            .with_context(|| format!("Could not receive user with id : {i}"))
    }

    pub fn get_all() -> Result<Vec<Self>, Error> {
        let mut connection = establish_connection()?;
        shopping_items
            .load::<ShoppingItem>(&mut connection)
            .context("Could not receive all shopping_items")
    }
}
