mod args;
use args::{DiktatorArgs, EntityType, UserCommand, UserSubcommand};
use clap::Parser;
use wg_page_backend::model::user::{User, UserInput};

fn main() {
    let args = DiktatorArgs::parse();

    match args.entity_type {
        EntityType::User(user) => handle_user_command(user),
    };
}

pub fn handle_user_command(user: UserCommand) {
    let command = user.command;
    match command {
        UserSubcommand::Create {
            name,
            email,
            pwhash,
        } => {
            let new_user = UserInput {
                name,
                email,
                pwhash,
            };
            let user = User::create(new_user);
            match user {
                Ok(_) => println!("Created User"),
                Err(_) => println!("Could not create User"),
            };
        }
        UserSubcommand::Update {
            email,
            name,
            pwhash,
        } => {
            let update_user = User {
                email,
                name,
                pwhash,
            };
            let user = User::update(update_user);
            match user {
                Ok(_) => println!("Updated User"),
                Err(_) => println!("Could not update User"),
            };
        }
        UserSubcommand::Delete { email } => {
            let user = User::delete(&email);
            match user {
                Ok(_) => println!("Deleted User"),
                Err(_) => println!("Could not delete User"),
            };
        }
        UserSubcommand::Show => {
            let user = User::get_all();
            match user {
                Ok(u) => {
                    for user in u {
                        println!("{user:?}");
                    }
                }
                Err(_) => println!("Could not query User"),
            };
        }
    }
}
