use refinery::{embed_migrations, Error as RefineryError};
use std::env;
use std::error::Error;
use postgres::{Client, NoTls, Error as PostgresError};
use std::fmt;

embed_migrations!("./files/migrations");

pub async fn run() -> Result<(), MigrateError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let mut client = Client::connect(&database_url, NoTls)?;

    migrations::runner().run(&mut client)?;
    Ok(())
}

#[derive(Debug)]
pub struct MigrateError {
    details: String
}

impl MigrateError {
    fn new(msg: &str) -> MigrateError {
        MigrateError{details: msg.to_string()}
    }
}

impl fmt::Display for MigrateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MigrateError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<PostgresError> for MigrateError {
    fn from(err: PostgresError) -> Self {
        MigrateError::new(&err.to_string())
    }
}

impl From<RefineryError> for MigrateError {
    fn from(err: RefineryError) -> Self {
        MigrateError::new(&err.to_string())
    }
}