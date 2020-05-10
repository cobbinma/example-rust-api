use std::env;
use sqlx::PgPool;

#[derive(Debug)]
pub(crate) struct State {
    pub db: sqlx::PgPool,
}

impl State {
    /// Create a new instance of `State`.
    pub(crate) async fn new() -> tide::Result<Self> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPool::new(&database_url).await?;
        
        Ok(Self { db: db_pool })
    }

}