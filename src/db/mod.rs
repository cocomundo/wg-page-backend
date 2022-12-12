use anyhow::{Context, Error};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod schema;

pub fn establish_connection() -> Result<PgConnection, Error> {
    dotenv().context("Failed to load environment")?;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .with_context(|| format!("Error connecting to {database_url}"))
}
