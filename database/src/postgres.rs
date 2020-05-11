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

    pub async fn get_pet(&self, id: i32) -> Result<Pet, DatabaseError> {
        let rec = sqlx::query!(
            r#"
                SELECT * FROM pets WHERE id = $1
            "#,
            id
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
        let mut tx = self.pool.begin().await?;
        sqlx::query("INSERT INTO pets (id, name, tag) VALUES ($1, $2, $3)")
            .bind(pet.id)
            .bind(pet.name.clone())
            .bind(pet.tag.clone())
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<Pet>, DatabaseError> {
        let mut pets = vec![];
        let recs = sqlx::query!(
            r#"
                SELECT id, name, tag
                    FROM pets
                ORDER BY id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        for rec in recs {
            pets.push(Pet {
                id: rec.id,
                name: rec.name,
                tag: rec.tag,
            });
        }

        Ok(pets)
    }
}
