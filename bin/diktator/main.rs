mod args;
use args::{DiktatorArgs, EntityType, UserCommand, UserSubcommand};
use clap::Parser;
use wg_page_backend::model::user::*;

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
            create_user(&n, &e);
        }
        UserSubcommand::Update {
            id: i,
            name: n,
            email: e,
        } => {
            update_user(i, n, e);
        }
        UserSubcommand::Delete { id: i } => {
            delete_user(i);
        }
        UserSubcommand::Show => {
            get_all();
        }
    }
}
