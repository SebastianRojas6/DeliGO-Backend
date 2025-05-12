use std::error::Error;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, NotSet, Set};
use application::DTOs::user_dto::CreateUserDTOs;
use application::mappers::db_mapper::DbMapper;
use application::repositories::user_fact_repository_abstract::UserFactRepositoryAbstract;
use domain::user::UserEntity;
use user::ActiveModel;
use crate::spi::db::db_mappers::UserFactDbMapper;
use crate::spi::db::db_models::user;
use crate::spi::db::db_models::user::Entity;

pub struct UserFactsRepository {
    db: DatabaseConnection,
}

impl UserFactsRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait(?Send)]
impl<'a> UserFactRepositoryAbstract for UserFactsRepository {
    async fn get_all_user_facts(&self) -> Result<Vec<UserEntity>, Box<dyn Error>> {
        let users = Entity::find().all(&self.db).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        let user_responses: Vec<UserEntity> = users.into_iter().map(|user| UserFactDbMapper::to_entity(user)).collect();
        Ok(user_responses)
    }

    async fn get_one_user_by_id_fact(&self, fact_id: i32) -> Result<UserEntity, Box<dyn Error>> {
        let user = Entity::find_by_id(fact_id).one(&self.db).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        match user {
            Some(user) => Ok(UserFactDbMapper::to_entity(user)),
            None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "User not found"))),
        }
    }

    async fn create_user_fact(&self, user: CreateUserDTOs) -> Result<UserEntity, Box<dyn Error>> {
        let new_user = ActiveModel {
            name: Set(user.name),
            phone: Set(user.phone),
            address: Set(user.address),
            ..Default::default()
        };
        let created_user = new_user.insert(&self.db).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(UserFactDbMapper::to_entity(created_user))
    }

    async fn update_user_fact(&self, user: application::DTOs::user_dto::UpdateUserDTOs) -> Result<UserEntity, Box<dyn Error>> {
        let user_to_update = Entity::find_by_id(user.id_user).one(&self.db).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        match user_to_update {
            Some(user_re ) => {
                let mut utu : ActiveModel = user_re.into();
                utu.name = match user.name { 
                    Some(name) => Set(name),
                    None => NotSet,
                };
                utu.phone = match user.phone { 
                    Some(phone) => Set(phone),
                    None => NotSet,
                };
                utu.address = match user.address { 
                    Some(address) => Set(address),
                    None => NotSet,
                };
                let updated_user = utu.update(&self.db).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
                Ok(UserFactDbMapper::to_entity(updated_user))
            }
            None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "User not found"))),
        }
    }

    async fn delete_user_by_id_fact(&self, fact_id: i32) -> Result<(), Box<dyn Error>> {
        let result = Entity::delete_by_id(fact_id).exec(&self.db).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;

        if result.rows_affected == 0 {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "User not found")))
        } else {
            Ok(())
        }
    }
}