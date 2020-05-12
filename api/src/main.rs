use async_std::task;
use dotenv::dotenv;

mod state;
mod handlers;

use state::State;

fn main() -> tide::Result<()> {
    task::block_on(async {
        dotenv().ok();
        env_logger::init();

        let state = State::new().await?;
        let mut app = tide::with_state(state);

        app.at("/pet").get(handlers::get_pets);
        app.at("/pet").post(handlers::create_pet);
        app.at("/pet/:id").get(handlers::get_pet);

        app.listen("127.0.0.1:8181").await?;
        Ok(())
    })
}
