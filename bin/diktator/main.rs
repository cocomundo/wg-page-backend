#[macro_use]
extern crate diesel;
extern crate dotenv;

mod args;
mod db;
mod model;
mod ops;

use args::EntityType;
use args::RustflixArgs;
use clap::Parser;
use ops::user::handle_user_command;

fn main() {
    let args = RustflixArgs::parse();

    match args.entity_type {
        EntityType::User(user) => handle_user_command(user),
    };
}

