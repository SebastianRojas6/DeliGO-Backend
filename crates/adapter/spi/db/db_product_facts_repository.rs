use crate::spi::db::db_mappers::ProductFactDbMapper;
use crate::spi::db::db_models::product::{ActiveModel, Entity};
use application::mappers::db_mapper::DbMapper;
use application::repositories::product_fact_repository_abstract::ProductFactRepositoryAbstract;
use async_trait::async_trait;
use domain::product::ProductEntity;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use std::error::Error;
use sea_orm::prelude::Decimal;
use application::dtos::product::product_in_dto::{ProductInDto, ProductUpdateInDto};

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

    async fn create_product_fact(&self, product: ProductInDto) -> Result<ProductEntity, Box<dyn Error>> {
        let new_product = ActiveModel {
            name: Set(product.name),
            price: Set(Decimal::try_from(product.price).unwrap()),  
            ..Default::default()
        };
        let created_product = new_product.insert(&self.db).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(ProductFactDbMapper::to_entity(created_product))
        
    }

    async fn update_product_fact(&self, product: ProductUpdateInDto) -> Result<ProductEntity, Box<dyn Error>> {
        let product_to_update = Entity::find_by_id(product.id_product).one(&self.db).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        let mut daa: ActiveModel = product_to_update.unwrap().into();
        if let Some(name) = product.name {
            daa.name = Set(name);
        }
        if let Some(price) = product.price {
            daa.price = Set(Decimal::try_from(price).unwrap());
        }
        let updated_product = daa.update(&self.db).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(ProductFactDbMapper::to_entity(updated_product))
    }
}
