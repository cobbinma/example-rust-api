use models::pet::Pet;
use sql_builder::prelude::*;
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;
use std::env;
use std::result::Result;

use crate::db_error::DatabaseError;

type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(Debug)]
pub struct Postgres {
    pool: sqlx::PgPool,
}

impl Postgres {
    pub async fn new() -> DatabaseResult<Postgres> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool = PgPool::new(&database_url).await?;

        Ok(Postgres { pool })
    }

    pub async fn get_pet(&self, id: i32) -> Result<Pet, DatabaseError> {
        let sql = SqlBuilder::select_from("pets")
            .fields(&["id", "name", "tag"])
            .and_where("id = ?".bind(&id))
            .sql()?;

        let pet = sqlx::query_as::<_, Pet>(&sql).fetch_one(&self.pool).await?;

        Ok(pet)
    }

    pub async fn create_pet(&self, pet: &Pet) -> DatabaseResult<()> {
        let sql = SqlBuilder::insert_into("pets")
            .field("id")
            .field("name")
            .field("tag")
            .values(&["$1, $2, $3"])
            .sql()?;

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

    pub async fn find_all(&self) -> DatabaseResult<Vec<Pet>> {
        let sql = SqlBuilder::select_from("pets")
            .fields(&["id", "name", "tag"])
            .order_by("id", false)
            .sql()?;

        let pets = sqlx::query_as::<_, Pet>(&sql).fetch_all(&self.pool).await?;

        Ok(pets)
    }
}
