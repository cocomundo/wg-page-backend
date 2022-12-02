use crate::args::{CreateUser, DeleteEntity, UpdateUser, UserCommand, UserSubcommand};
use crate::db::establish_connection;
use crate::db::schema::users::dsl::*;
use crate::model::user::{NewUser, User as DBUser};
use diesel::prelude::*;

pub fn handle_user_command(user: UserCommand) {
    let command = user.command;
    match command {
        UserSubcommand::Create(user) => {
            create_user(user);
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

fn create_user(user: CreateUser) {
    println!("Creating user: {:?}", user);

    let mut connection = establish_connection();
    let new_user = NewUser {
        name: &user.name,
        email: &user.email,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut connection)
        .expect("Error saving new user");
}

fn update_user(user: UpdateUser) {
    println!("Updating user: {:?}", user);

    let mut connection = establish_connection();
    let db_user = DBUser {
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
    let results = users.load::<DBUser>(&mut connection).unwrap();

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:?}", user);
    }
}
