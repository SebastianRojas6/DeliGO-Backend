use crate::spi::db::db_mappers::ProductFactDbMapper;
use crate::spi::db::db_models::product::Entity;
use application::mappers::db_mapper::DbMapper;
use application::repositories::product_fact_repository_abstract::ProductFactRepositoryAbstract;
use async_trait::async_trait;
use domain::product::ProductEntity;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::error::Error;

pub struct ProductFactsRepository {
    db: DatabaseConnection,
}

impl ProductFactsRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait(?Send)]
impl<'a> ProductFactRepositoryAbstract for ProductFactsRepository {
    async fn get_one_product_by_id_fact(&self, fact_id: i32) -> Result<ProductEntity, Box<dyn Error>> {
        let product = Entity::find_by_id(fact_id).one(&self.db).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        match product {
            Some(product) => Ok(ProductFactDbMapper::to_entity(product)),
            None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Product not found"))),
        }
    }

    async fn get_all_product_facts(&self) -> Result<Vec<ProductEntity>, Box<dyn Error>> {
        let products = Entity::find().all(&self.db).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        let product_responses: Vec<ProductEntity> = products.into_iter().map(|product| ProductFactDbMapper::to_entity(product)).collect();
        Ok(product_responses)
    }

    async fn delete_product_by_id_fact(&self, fact_id: i32) -> Result<(), Box<dyn Error>> {
        let result = Entity::delete_by_id(fact_id).exec(&self.db).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        if result.rows_affected == 0 {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Product not found")))
        } else {
            Ok(())
        }
    }
}
