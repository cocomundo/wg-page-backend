use crate::db::establish_connection;
use crate::db::schema::shopping_items;
use crate::db::schema::shopping_items::dsl::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable)]
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
    pub fn create(new_item: NewShoppingItem) {
        let mut connection = establish_connection();
        diesel::insert_into(shopping_items)
            .values(&new_item)
            .execute(&mut connection)
            .expect("Error saving new user");
    }

    pub fn update(item: ShoppingItem) {
        let mut connection = establish_connection();

        diesel::update(shopping_items.find(item.id))
            .set(&item)
            .execute(&mut connection)
            .expect("Error updating user");
    }

    pub fn delete(i: i32) {
        let mut connection = establish_connection();
        diesel::delete(shopping_items.find(i))
            .execute(&mut connection)
            .expect("Error deleting user");
    }

    pub fn get(i: i32) {
        let mut connection = establish_connection();
        let result = shopping_items
            .filter(shopping_items::id.eq(i))
            .load::<ShoppingItem>(&mut connection)
            .unwrap();
        println!("{:?}", result);
    }

    pub fn get_all() {
        let mut connection = establish_connection();
        let results = shopping_items
            .load::<ShoppingItem>(&mut connection)
            .unwrap();

        println!("Displaying {} users", results.len());
        for user in results {
            println!("{:?}", user);
        }
    }
}
