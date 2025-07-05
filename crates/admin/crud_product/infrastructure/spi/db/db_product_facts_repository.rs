use std::error::Error;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use crate::crud_product::application::dto::product_dto::{ProductCreateDto, ProductUpdateDto};
use crate::crud_product::domain::models::product::ProductEntity;
use crate::crud_product::domain::repository::ProductAbstractRepository;
use crate::crud_product::infrastructure::spi::db::db_mapper::DBMapper;
use crate::crud_product::infrastructure::spi::db::product_entity::{ActiveModel, Entity};

pub struct DbProductFactsRepository {
    connection: DatabaseConnection,
}

impl DbProductFactsRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        DbProductFactsRepository { connection }
    }

    pub async fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}

#[async_trait::async_trait(?Send)]
impl<'a> ProductAbstractRepository for DbProductFactsRepository {
    async fn get_all_products(&self) -> Result<Vec<ProductEntity>, Box<dyn Error>> {
        let products = Entity::find().all(&self.connection).await.map_err(|e| {
            Box::new(e) as Box<dyn Error>
        })?;
        let product_response = products.into_iter().map(|product| DBMapper::to_entity(product)).collect();
        Ok(product_response)
    }

    async fn get_product_by_id(&self, id: i32) -> Result<ProductEntity, Box<dyn Error>> {
        let product = Entity::find_by_id(id).one(&self.connection).await.map_err(|e| {Box::new(e) as Box<dyn Error>})?;
        match product {
            Some(product) => Ok(DBMapper::to_entity(product)),
            None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Product not found")) as Box<dyn Error>)
        }
    }

    async fn create_product(&self, product: ProductCreateDto) -> Result<ProductEntity, Box<dyn Error>> {
        let new_product = ActiveModel {
            name: sea_orm::Set(product.name),
            price: sea_orm::Set(sea_orm::prelude::Decimal::try_from(product.price).unwrap_or_default()),
            ..Default::default()
        };
        let inserted_product = new_product.insert(&self.connection).await.map_err(|e| {
            Box::new(e) as Box<dyn Error>
        })?;
        let product_entity = DBMapper::to_entity(inserted_product);
        Ok(product_entity)
    }

    async fn update_product(&self, product: ProductUpdateDto) -> Result<ProductEntity, Box<dyn Error>> {
        let product_to_update = Entity::find_by_id(product.id_product)
            .one(&self.connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        let mut updated: ActiveModel = product_to_update.unwrap().into();
        if let Some(name) = product.name {
            updated.name = sea_orm::Set(name);
        }
        if let Some(price) = product.price {
            updated.price = sea_orm::Set(sea_orm::prelude::Decimal::try_from(price).unwrap_or_default());
        }
        let updated_product = updated.update(&self.connection).await.map_err(|e| {
            Box::new(e) as Box<dyn Error>
        })?;
        let product_entity = DBMapper::to_entity(updated_product);
        Ok(product_entity)
    }

    async fn delete_product(&self, id: i32) -> Result<(), Box<dyn Error>> {
        let product_to_delete = Entity::find_by_id(id)
            .one(&self.connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        if let Some(product) = product_to_delete {
            let active_model: ActiveModel = product.into();
            active_model.delete(&self.connection).await.map_err(|e| {
                Box::new(e) as Box<dyn Error>
            })?;
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Product not found")) as Box<dyn Error>)
        }
    }
}