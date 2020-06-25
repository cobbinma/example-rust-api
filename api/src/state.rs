use async_std::prelude::*;
use database::postgres::Postgres;
use models::repository::Repository;

pub(crate) struct State {
    db: Box<dyn Repository + Send + Sync + 'static>,
}

impl State {
    pub(crate) async fn new() -> tide::Result<Self> {
        let (db, ()) = Postgres::new().join(database::migration::run()).await;

        Ok(Self { db: Box::new(db) })
    }

    pub fn db(&self) -> &Box<dyn Repository + Send + Sync + 'static> {
        &self.db
    }
}
