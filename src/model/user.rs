use crate::db::schema::users;
use serde::{Deserialize, Serialize};

use crate::args::{DeleteEntity, UpdateUser, UserCommand, UserSubcommand};
use crate::db::establish_connection;
use crate::db::schema::users::dsl::*;
use diesel::prelude::*;

// used for insertin
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

// used for updating and querying
#[derive(Serialize, Deserialize, Queryable, Debug, AsChangeset, Eq, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub fn handle_user_command(user: UserCommand) {
    let command = user.command;
    match command {
        UserSubcommand::Create{name:n, email:e} => {
            create_user(&n, &e);
        }
        UserSubcommand::Update(user) => {
            update_user(user);
        }
        UserSubcommand::Delete(delete_entity) => {
            delete_user(delete_entity);
        }
        UserSubcommand::Show => {
            show_users();
        }
    }
}

fn create_user(n: &str, e: &str) {
    let mut connection = establish_connection();
    let new_user = NewUser {
        name:n,
        email:e,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut connection)
        .expect("Error saving new user");
}

fn update_user(user: UpdateUser) {
    println!("Updating user: {:?}", user);

    let mut connection = establish_connection();
    let db_user = User {
        id: user.id,
        name: user.name,
        email: user.email,
    };

    diesel::update(users.find(user.id))
        .set(&db_user)
        .execute(&mut connection)
        .expect("Error updating user");
}

fn delete_user(user: DeleteEntity) {
    println!("Deleting user: {:?}", user);

    let mut connection = establish_connection();
    diesel::delete(users.find(user.id))
        .execute(&mut connection)
        .expect("Error deleting user");
}

fn show_users() {
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
