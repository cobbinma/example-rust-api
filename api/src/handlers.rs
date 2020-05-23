use models::pet::Pet;
use tide::{Request, Response, StatusCode};
use log::{warn, trace};

use crate::state::State;
use crate::error_response::ErrorResponse;

pub(crate) async fn get_pet(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let id: i32 = match req.param("id") {
        Ok(id) => id,
        Err(e) => {
            trace!("Bad Request: {:?}", e);
            return Ok(Response::new(StatusCode::BadRequest).body_json(&ErrorResponse::from(e))?)
        }
    };

    match req.state().db().get_pet(id).await {
        Ok(pet) => {
            Ok(Response::new(StatusCode::Ok).body_json(&pet)?)
        }
        Err(e) => {
            warn!("Error getting pet from database: {:?}", e);
            Ok(Response::new(StatusCode::InternalServerError).body_json(&ErrorResponse::from(e))?)
        }
    }
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
