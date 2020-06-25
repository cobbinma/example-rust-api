use async_std::prelude::*;
use async_std::task;
use database::postgres::Postgres;
use dotenv::dotenv;
use tide::log;

mod error_response;
mod handlers;
mod server;
mod state;

use server::get_server;
use state::State;

fn main() -> tide::Result<()> {
    task::block_on(async {
        femme::ndjson::Logger::new()
            .start(log::Level::Info.to_level_filter())
            .unwrap();
        dotenv().ok();

        let (db, ()) = Postgres::new().join(database::migration::run()).await;

        let app = get_server(Box::new(db)).await?;

        app.listen("127.0.0.1:8181").await?;
        Ok(())
    })
}
