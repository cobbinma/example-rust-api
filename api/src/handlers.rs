use models::pet::Pet;
use tide::{Request, Response, StatusCode};

use crate::state::State;

pub(crate) async fn get_pet(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let id: i32 = req.param("id")?;
    let pet = req.state().db().get_pet(id).await?;

    Ok(Response::new(StatusCode::Ok).body_json(&pet)?)
}

pub(crate) async fn get_pets(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let pets = req.state().db().find_all().await?;

    Ok(Response::new(StatusCode::Ok).body_json(&pets)?)
}

pub(crate) async fn create_pet(mut req: Request<State>) -> tide::Result<impl Into<Response>> {
    let pet: Pet = req.body_json().await?;
    req.state().db().create_pet(&pet).await?;

    Ok(Response::new(StatusCode::Created))
}
