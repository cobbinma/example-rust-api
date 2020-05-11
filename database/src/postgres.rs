use models::pet::Pet;
use sqlx::PgPool;
use std::env;

use crate::db_error::DatabaseError;

#[derive(Debug)]
pub struct Postgres {
    pool: sqlx::PgPool,
}

impl Postgres {
    pub async fn new() -> Result<Postgres, DatabaseError> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool = PgPool::new(&database_url).await?;

        Ok(Postgres { pool })
    }

    pub async fn get_pet(&self, id: i64) -> Result<Pet, DatabaseError> {
        let rec = sqlx::query!(
            r#"
            SELECT * FROM pets WHERE id = $1
        "#,
            id as i32
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Pet {
            id: rec.id,
            name: rec.name,
            tag: rec.tag,
        })
    }

    pub async fn create_pet(&self, pet: &Pet) -> Result<(), DatabaseError> {
        unimplemented!();
    }
}
