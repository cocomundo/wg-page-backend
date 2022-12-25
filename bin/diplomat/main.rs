use std::net::SocketAddr;

use clap::Parser;
use reqwest::{blocking::Client, header::CONTENT_TYPE, StatusCode};
use wg_page_backend::{
    args::{Arguments, Entity, UserCommand, UserSubcommand},
    model::user::{NewUser, User},
};

fn main() {
    let args = Arguments::parse();

    match args.entity {
        Entity::User(user) => handle_user_command(user, &args.address),
    };
}

pub fn handle_user_command(user: UserCommand, addr: &SocketAddr) {
    let command = user.command;
    match command {
        UserSubcommand::Create { name, email } => {
            let new_user = NewUser { name, email };
            let client = Client::new();
            let res = client
                .post(format!("http://{addr}/user")) // TODO this is probaby wrong approach but works
                .header(CONTENT_TYPE, "application/json")
                .body(serde_json::to_string(&new_user).unwrap())
                .send();

            match res {
                Ok(r) => println!("Created User, response: {r:#?}"),
                Err(e) => println!("Could not create User, error: {e:#?}"),
            };
        }
        UserSubcommand::Update { id, name, email } => {
            let client = Client::new();
            let old_user: User = client
                .get(format!("http://{addr}/user/{id}"))
                .send()
                .expect(&format!("Failed to get user with id {id}"))
                .json()
                .unwrap();
            let name = name.unwrap_or(old_user.name);
            let email = email.unwrap_or(old_user.email);
            let update_user = User { id, name, email };
            let res = client
                .put(format!("http://{addr}/user/{id}"))
                .header(CONTENT_TYPE, "application/json")
                .body(serde_json::to_string(&update_user).unwrap())
                .send();
            match res {
                Ok(r) => {
                    let user: User = r.json().unwrap();
                    println!("Updated user, {user:#?}");
                }
                Err(e) => println!("Could not update user, {e:#?}"),
            };
        }
        UserSubcommand::Get { id } => {
            let client = Client::new();
            let res = client.get(format!("http://{addr}/user/{id}")).send();

            match res {
                Ok(r) => {
                    let user: User = r.json().unwrap();
                    println!("Got user: {user:#?}");
                }
                Err(e) => println!("Could not get user, error: {e:#?}"),
            }
        }
        UserSubcommand::Delete { id } => {
            let client = Client::new();
            let res = client.delete(format!("http://{addr}/user/{id}")).send();

            match res {
                Ok(r) => println!("Deleted user, response: {r:#?}"),
                Err(e) => println!("Could not delete user, error: {e:#?}"),
            };
        }
        UserSubcommand::Show => {
            let client = Client::new();
            let res = client
                .get(format!("http://{addr}/user"))
                .header(CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            match res.status() {
                StatusCode::OK => {
                    match res.json::<Vec<User>>() {
                        Ok(users) => println!("Users: {users:#?}"),
                        Err(e) => println!("Error: {e:#?}"),
                    };
                }
                e => println!("{e:#?}"),
            }
        }
    }
}
