use postgres::{Client, NoTls};
use refinery::embed_migrations;
use std::env;

use crate::db_error::DatabaseError;

embed_migrations!("files/migrations");

pub async fn run() -> Result<(), DatabaseError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let mut client = Client::connect(&database_url, NoTls)?;

    migrations::runner().run(&mut client)?;
    Ok(())
}
