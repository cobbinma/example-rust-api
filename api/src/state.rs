use database;
use database::postgres::Postgres;

#[derive(Debug)]
pub(crate) struct State {
    pub db: Postgres,
}

impl State {
    pub(crate) async fn new() -> tide::Result<Self> {
        database::migration::run().await?;

        let db = Postgres::new().await?;

        Ok(Self { db })
    }
}
