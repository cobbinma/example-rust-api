use sqlx::PgPool;
use std::env;

use crate::db_error::DatabaseError;

#[derive(Debug)]
pub struct Postgres {
    pool: sqlx::PgPool,
}

impl Postgres {
    pub async fn new() -> Result<Postgres, DatabaseError> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool = PgPool::new(&database_url).await?;

        Ok(Postgres { pool })
    }
}
