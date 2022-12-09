use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct RustflixArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, update, delete or show users
    User(UserCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// Create a new user
    Create { name: String, email: String },

    /// Update an existing user
    Update{id: i32, name: String, email: String },

    /// Delete a user
    Delete{id: i32},

    /// Show all users
    Show,
}

