use crate::db::establish_connection;
use crate::db::schema::users;
use crate::db::schema::users::dsl::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// used for insertin
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

// used for updating and querying
#[derive(Serialize, Deserialize, Queryable, Debug, AsChangeset, Eq, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub fn create_user(n: &str, e: &str) {
    let mut connection = establish_connection();
    let new_user = NewUser {
        name: n.to_string(),
        email: e.to_string(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut connection)
        .expect("Error saving new user");
}

pub fn update_user(i: i32, n: String, e: String) {
    let mut connection = establish_connection();
    let db_user = User {
        id: i,
        name: n,
        email: e,
    };

    diesel::update(users.find(i))
        .set(&db_user)
        .execute(&mut connection)
        .expect("Error updating user");
}

pub fn delete_user(i: i32) {
    let mut connection = establish_connection();
    diesel::delete(users.find(i))
        .execute(&mut connection)
        .expect("Error deleting user");
}

pub fn show_users() {
    let mut connection = establish_connection();
    let results = users.load::<User>(&mut connection).unwrap();

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:?}", user);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialization() {
        let user = User {
            id: 21,
            name: "otto".to_string(),
            email: "otto@gmail.com".to_string(),
        };
        let json = dbg!(serde_json::to_string(&user).unwrap());
        let test_user: User = serde_json::from_str(&json).unwrap();
        assert_eq!(test_user, user);
    }
}
