use models::pet::Pet;
use sql_builder::prelude::*;
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;
use std::env;
use std::error::Error;
use std::result::Result;
use models::repository::Repository;
use async_trait::async_trait;

use crate::db_error::DatabaseError;

#[derive(Debug)]
pub struct Postgres {
    pool: sqlx::PgPool,
}

impl Postgres {
    pub async fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool = PgPool::new(&database_url).await.expect("could not create postgres connnection pool");

        Postgres { pool }
    }

}

#[async_trait]
impl Repository for Postgres {
    async fn get_pet(&self, id: i32) -> Result<Pet, Box<dyn Error>> {
        let sql = SqlBuilder::select_from("pets")
            .fields(&["id", "name", "tag"])
            .and_where("id = ?".bind(&id))
            .sql().map_err(DatabaseError::from)?;

        let pet = sqlx::query_as::<_, Pet>(&sql).fetch_one(&self.pool).await?;

        Ok(pet)
    }

    async fn create_pet(&self, pet: &Pet) -> Result<(), Box<dyn Error>> {
        let sql = SqlBuilder::insert_into("pets")
            .field("id")
            .field("name")
            .field("tag")
            .values(&["$1, $2, $3"])
            .sql().map_err(DatabaseError::from)?;

        let mut tx = self.pool.begin().await?;

        sqlx::query(&sql)
            .bind(&pet.id)
            .bind(&pet.name)
            .bind(&pet.tag)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Pet>, Box<dyn Error>> {
        let sql = SqlBuilder::select_from("pets")
            .fields(&["id", "name", "tag"])
            .order_by("id", false)
            .sql().map_err(DatabaseError::from)?;

        let pets = sqlx::query_as::<_, Pet>(&sql).fetch_all(&self.pool).await?;

        Ok(pets)
    }
}