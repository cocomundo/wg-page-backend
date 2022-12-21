use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct DiktatorArgs {
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
    Create { email: String, name: String, pwhash: String},

    /// Update an existing user
    Update {
        email: String,
        name: String,
        pwhash: String,
    },

    /// Delete a user
    Delete { email: String },

    /// Show all users
    Show,
}
