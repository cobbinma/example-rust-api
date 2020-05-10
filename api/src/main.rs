use dotenv::dotenv;
use async_std::task;
use database;

mod state;

use state::State;

fn main() -> tide::Result<()> {
    task::block_on(async {
        dotenv().ok();
        database::migration::run().await?;
        
        let state = State::new().await?;
        let mut app = tide::with_state(state);

        app.at("/").get(|_| async { Ok("Hello, world!") });

        app.listen("127.0.0.1:8181").await?;
        Ok(())
    })
}
