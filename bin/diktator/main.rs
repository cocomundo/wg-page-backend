mod args;
use args::{DiktatorArgs, EntityType, UserCommand, UserSubcommand};
use clap::Parser;
use wg_page_backend::model::user::{NewUser, User};

fn main() {
    let args = DiktatorArgs::parse();

    match args.entity_type {
        EntityType::User(user) => handle_user_command(user),
    };
}

pub fn handle_user_command(user: UserCommand) {
    let command = user.command;
    match command {
        UserSubcommand::Create { name: n, email: e } => {
            let new_user = NewUser { name: n, email: e };
            User::create(new_user);
        }
        UserSubcommand::Update {
            id: i,
            name: n,
            email: e,
        } => {
            let update_user = User {
                id: i,
                name: n,
                email: e,
            };
            User::update(update_user);
        }
        UserSubcommand::Delete { id: i } => {
            User::delete(i);
        }
        UserSubcommand::Show => {
            User::get_all();
        }
    }
}
