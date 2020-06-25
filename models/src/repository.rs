use async_trait::async_trait;
use std::error::Error;

use crate::pet::Pet;

#[async_trait]
pub trait Repository {
    async fn get_pet(&self, id: i32) -> Result<Pet, Box<dyn Error>>;
    async fn create_pet(&self, pet: &Pet) -> Result<(), Box<dyn Error>>;
    async fn find_all(&self) -> Result<Vec<Pet>, Box<dyn Error>>;
}