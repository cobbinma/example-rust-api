use log::{trace, warn};
use models::pet::Pet;
use tide::{Body, Request, Response, StatusCode};

use crate::error_response::ErrorResponse;
use crate::state::State;

pub(crate) async fn get_pet(req: Request<State>) -> tide::Result {
    let id: i32 = match req.param("id") {
        Ok(id) => id,
        Err(e) => {
            trace!("Bad Request: {:?}", e);
            let mut res = Response::new(StatusCode::BadRequest);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            return Ok(res);
        }
    };

    match req.state().db().get_pet(id).await {
        Ok(pet) => {
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(Body::from_json(&pet)?);
            Ok(res)
        }
        Err(e) => {
            warn!("Error getting pet from database: {:?}", e);
            let mut res = Response::new(StatusCode::InternalServerError);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            Ok(res)
        }
    }
}

pub(crate) async fn get_pets(req: Request<State>) -> tide::Result {
    match req.state().db().find_all().await {
        Ok(pets) => {
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(Body::from_json(&pets)?);
            Ok(res)
        }
        Err(e) => {
            warn!("Error getting pets from database: {:?}", e);
            let mut res = Response::new(StatusCode::InternalServerError);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            Ok(res)
        }
    }
}

pub(crate) async fn create_pet(mut req: Request<State>) -> tide::Result<Response> {
    let pet: Pet = match req.body_json().await {
        Ok(pet) => pet,
        Err(e) => {
            trace!("Bad Request: {:?}", e);
            let mut res = Response::new(StatusCode::BadRequest);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            return Ok(res);
        }
    };

    match req.state().db().create_pet(&pet).await {
        Ok(()) => Ok(Response::new(StatusCode::Created)),
        Err(e) => {
            warn!("Error creating pet from database: {:?}", e);
            let mut res = Response::new(StatusCode::InternalServerError);
            res.set_body(Body::from_json(&ErrorResponse::from(e))?);
            Ok(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use http_types::Url;
    use mockall::predicate::*;
    use mockall::*;
    use models::pet::Pet;
    use models::repository::Repository;
    use serde_json::json;
    use std::error::Error;
    use tide::Response;

    use crate::server::get_app;

    mock! {
        pub Database {
            fn get_pet(&self, id: i32) -> Result<Pet, Box<dyn Error>> {}
            fn create_pet(&self, pet: &Pet) -> Result<(), Box<dyn Error>> {}
            fn find_all(&self) -> Result<Vec<Pet>, Box<dyn Error>> {}
        }
    }

    #[async_trait]
    impl Repository for MockDatabase {
        async fn get_pet(&self, id: i32) -> Result<Pet, Box<dyn Error>> {
            self.get_pet(id)
        }
        async fn create_pet(&self, pet: &Pet) -> Result<(), Box<dyn Error>> {
            self.create_pet(pet)
        }
        async fn find_all(&self) -> Result<Vec<Pet>, Box<dyn Error>> {
            self.find_all()
        }
    }

    #[async_std::test]
    async fn test_get_pet() {
        let id: i32 = 1;
        let name = "Tom";
        let mut mock_db = MockDatabase::default();
        mock_db
            .expect_get_pet()
            .with(predicate::eq(id))
            .times(1)
            .returning(move |_| {
                Ok(Pet {
                    id,
                    name: String::from(name),
                    tag: None,
                })
            });
        let app = get_app(Box::new(mock_db))
            .await
            .expect("could not create app");
        let mut req = http_types::Request::new(
            http_types::Method::Get,
            Url::parse("http://localhost:8181/pet/1").unwrap(),
        );
        req.set_body(
            json!({
                "id": id,
                "name": name
            })
            .to_string(),
        );
        let mut res: Response = app.respond(req).await.unwrap();

        assert_eq!(200, res.status());
        let body = res.take_body().into_string().await.unwrap();

        if let Ok(pet) = serde_json::from_str::<Pet>(&body) {
            assert_eq!(id, pet.id);
            assert_eq!(name, pet.name);
            assert_eq!(Option::None, pet.tag);
        };
    }

    #[async_std::test]
    async fn test_get_pets() {
        let id: i32 = 1;
        let name = "Tom";
        let mut mock_db = MockDatabase::default();
        mock_db.expect_find_all().times(1).returning(move || {
            Ok(vec![Pet {
                id,
                name: String::from(name),
                tag: Option::None,
            }])
        });
        let app = get_app(Box::new(mock_db))
            .await
            .expect("could not create app");
        let req = http_types::Request::new(
            http_types::Method::Get,
            Url::parse("http://localhost:8181/pets").unwrap(),
        );

        let mut res: Response = app.respond(req).await.unwrap();

        assert_eq!(200, res.status());

        let body = res.take_body().into_string().await.unwrap();
        if let Ok(pets) = serde_json::from_str::<Vec<Pet>>(&body) {
            assert_eq!(1, pets.len());
            assert_eq!(id, pets[0].id);
            assert_eq!(name, pets[0].name);
            assert_eq!(Option::None, pets[0].tag);
        };
    }

    #[async_std::test]
    async fn test_create_pet() {
        let id: i32 = 1;
        let name = "Tom";
        let mut mock_db = MockDatabase::default();
        mock_db
            .expect_create_pet()
            .with(predicate::eq(Pet {
                id,
                name: String::from(name),
                tag: Option::None,
            }))
            .times(1)
            .returning(move |_| Ok(()));
        let app = get_app(Box::new(mock_db))
            .await
            .expect("could not create app");
        let mut req = http_types::Request::new(
            http_types::Method::Post,
            Url::parse("http://localhost:8181/pet").unwrap(),
        );
        req.set_body(
            json!({
                "id": id,
                "name": name
            })
            .to_string(),
        );

        let mut res: Response = app.respond(req).await.unwrap();

        assert_eq!(201, res.status());
        let body = res.take_body().into_string().await.unwrap();
        assert_eq!("", body);
    }
}
