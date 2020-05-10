use std::env;
use sqlx::PgPool;
use postgres::{Client, NoTls};
use async_std::task;

#[derive(Debug)]
pub(crate) struct State {
    pub db: sqlx::PgPool,
}

impl State {
    /// Create a new instance of `State`.
    pub(crate) async fn new() -> tide::Result<Self> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPool::new(&database_url).await?;

        let mut client = Client::connect(&database_url, NoTls)?;

        task::block_on(async {
            database::migration::migrations::runner().run(&mut client).unwrap();
        });
        
        Ok(Self { db: db_pool })
    }

}