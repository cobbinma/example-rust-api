use async_std::prelude::*;
use database::postgres::Postgres;

#[derive(Debug)]
pub(crate) struct State {
    db: Postgres,
}

impl State {
    pub(crate) async fn new() -> tide::Result<Self> {
        let (db, ()) = Postgres::new().try_join(database::migration::run()).await?;

        Ok(Self { db })
    }

    pub fn db(&self) -> &Postgres {
        &self.db
    }
}
