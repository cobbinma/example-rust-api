use async_std::task;
use dotenv::dotenv;

mod state;

use state::State;

fn main() -> tide::Result<()> {
    task::block_on(async {
        dotenv().ok();

        let state = State::new().await?;
        let mut app = tide::with_state(state);

        app.at("/").get(|_| async { Ok("Hello, world!") });

        app.listen("127.0.0.1:8181").await?;
        Ok(())
    })
}
