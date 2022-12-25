use anyhow::Context;
use clap::{Args, Parser, Subcommand};
use std::net::{SocketAddr, ToSocketAddrs};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Arguments {
    /// IP address
    #[arg(
        short,
        long,
        default_value = "127.0.0.1:8000",
        value_parser(resolve_address)
    )]
    pub address: SocketAddr,

    #[command(subcommand)]
    pub entity: Entity,
}

#[derive(Debug, Subcommand)]
pub enum Entity {
    /// Create, update, delete or show users
    User(UserCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[command(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// Create a new user
    Create {
        /// User name
        #[arg(short, long)]
        name: String,

        /// User email
        #[arg(short, long)]
        email: String,
    },

    /// Get a user by ID
    Get { id: i32 },

    /// Update an existing user
    Update {
        /// Id of user to update
        #[arg(short, long)]
        id: i32,

        /// New user name
        #[arg(short, long)]
        name: Option<String>,

        /// New user email
        #[arg(short, long)]
        email: Option<String>,
    },

    /// Delete a user
    Delete { id: i32 },

    /// Show all users
    Show,
}

fn resolve_address(address: &str) -> Result<SocketAddr, anyhow::Error> {
    address
        .to_socket_addrs()
        .map(|mut v| v.next().expect("No address parsed"))
        .with_context(|| format!("Failed to parse socket address {address}"))
}
