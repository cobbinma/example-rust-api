use log::{trace, warn};
use models::pet::Pet;
use tide::{Request, Response, StatusCode};

use crate::error_response::ErrorResponse;
use crate::state::State;

pub(crate) async fn get_pet(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let id: i32 = match req.param("id") {
        Ok(id) => id,
        Err(e) => {
            trace!("Bad Request: {:?}", e);
            return Ok(Response::new(StatusCode::BadRequest).body_json(&ErrorResponse::from(e))?);
        }
    };

    match req.state().db().get_pet(id).await {
        Ok(pet) => Ok(Response::new(StatusCode::Ok).body_json(&pet)?),
        Err(e) => {
            warn!("Error getting pet from database: {:?}", e);
            Ok(
                Response::new(StatusCode::InternalServerError)
                    .body_json(&ErrorResponse::from(e))?,
            )
        }
    }
}

pub(crate) async fn get_pets(req: Request<State>) -> tide::Result<impl Into<Response>> {
    match req.state().db().find_all().await {
        Ok(pets) => Ok(Response::new(StatusCode::Ok).body_json(&pets)?),
        Err(e) => {
            warn!("Error getting pets from database: {:?}", e);
            Ok(
                Response::new(StatusCode::InternalServerError)
                    .body_json(&ErrorResponse::from(e))?,
            )
        }
    }
}

pub(crate) async fn create_pet(mut req: Request<State>) -> tide::Result<impl Into<Response>> {
    let pet: Pet = match req.body_json().await {
        Ok(pet) => pet,
        Err(e) => {
            trace!("Bad Request: {:?}", e);
            return Ok(Response::new(StatusCode::BadRequest).body_json(&ErrorResponse::from(e))?);
        }
    };

    match req.state().db().create_pet(&pet).await {
        Ok(()) => Ok(Response::new(StatusCode::Created)),
        Err(e) => {
            warn!("Error creating pet from database: {:?}", e);
            Ok(
                Response::new(StatusCode::InternalServerError)
                    .body_json(&ErrorResponse::from(e))?,
            )
        }
    }
}
