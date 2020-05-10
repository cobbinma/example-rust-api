use database;
use database::postgres::Postgres;
use async_std::prelude::*;

#[derive(Debug)]
pub(crate) struct State {
    pub db: Postgres,
}

impl State {
    pub(crate) async fn new() -> tide::Result<Self> {
        let (db, ()) = Postgres::new().try_join(database::migration::run()).await?;

        Ok(Self { db })
    }
}
