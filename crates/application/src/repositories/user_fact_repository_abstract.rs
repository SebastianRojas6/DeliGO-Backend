#[cfg(test)]
use mockall::{predicate::*, automock};
use std::error::Error;
use async_trait::async_trait;
use domain::user::UserEntity;
use crate::DTOs::user::user_in_dto::{UserCreateInDTO, UserUpdateInDTO};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait UserFactRepositoryAbstract {
    async fn get_all_user_facts(&self) -> Result<Vec<UserEntity>, Box<dyn Error>>;
    async fn get_one_user_by_id_fact(&self, fact_id: i32) -> Result<UserEntity, Box<dyn Error>>;
    async fn create_user_fact(&self, user_fact: UserCreateInDTO) -> Result<UserEntity, Box<dyn Error>>;
    async fn update_user_fact(&self, fact_id: i32, user_fact: UserUpdateInDTO) -> Result<UserEntity, Box<dyn Error>>;
    async fn delete_user_by_id_fact(&self, user_fact_id: i32) -> Result<(), Box<dyn Error>>;
}