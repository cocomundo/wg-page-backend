use std::net::{SocketAddr, ToSocketAddrs};

use anyhow::Context;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// IP address
    #[clap(
        short,
        long,
        default_value = "localhost:8000",
        value_parser(resolve_address)
    )]
    pub address: SocketAddr,
}

fn resolve_address(address: &str) -> Result<SocketAddr, anyhow::Error> {
    address
        .to_socket_addrs()
        .map(|mut v| v.next().expect("No address parsed"))
        .with_context(|| format!("Failed to parse socket address {address}"))
}
