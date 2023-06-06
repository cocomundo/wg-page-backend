use clap::Parser;
use wg_page_backend::{
    args::{Arguments, Entity, UserCommand, UserSubcommand},
    model::user::{NewUser, User},
};

fn main() {
    let args = Arguments::parse();

    match args.entity {
        Entity::User(user) => handle_user_command(user),
    };
}

pub fn handle_user_command(user: UserCommand) {
    let command = user.command;
    match command {
        UserSubcommand::Create { name: n, email: e } => {
            let new_user = NewUser { name: n, email: e };
            let user = User::create(new_user);
            match user {
                Ok(_) => println!("Created User"),
                Err(_) => println!("Could not create User"),
            };
        }
        UserSubcommand::Update { id, name, email } => {
            let old_user: User = User::get(id).expect(&format!("No user with id {id}"));
            let name = name.unwrap_or(old_user.name);
            let email = email.unwrap_or(old_user.email);
            let update_user = User { id, name, email };
            let user = User::update(update_user);
            match user {
                Ok(u) => println!("Updated user: {u:#?}"),
                Err(e) => println!("Could not update user, error {e:#?}"),
            };
        }
        UserSubcommand::Delete { id: i } => {
            let user = User::delete(i);
            match user {
                Ok(_) => println!("Deleted User"),
                Err(_) => println!("Could not delete User"),
            };
        }
        UserSubcommand::Get { id } => {
            let user = User::get(id);
            match user {
                Ok(u) => println!("Got user: {u:#?}"),
                Err(e) => println!("Could not get user, {e:#?}"),
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
