use models::repository::Repository;
use std::fs;
use tide::{Response, Server, StatusCode};

use crate::handlers;
use crate::State;

pub(crate) async fn get_app(
    repository: Box<dyn Repository + Send + Sync + 'static>,
) -> tide::Result<Server<State>> {
    let state = State::new(repository).await?;
    let mut app = tide::with_state(state);

    app.at("/pets").get(handlers::get_pets);
    app.at("/pet").post(handlers::create_pet);
    app.at("/pet/:id").get(handlers::get_pet);

    app.at("/healthz")
        .get(|_| async { Ok(Response::new(StatusCode::Ok)) });
    app.at("/oas")
        .get(|_| async { Ok(fs::read_to_string("oas/v1.yaml")?) });

    Ok(app)
}
