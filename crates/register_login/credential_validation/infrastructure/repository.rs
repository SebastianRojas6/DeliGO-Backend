use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};

use crate::credential_validation::domain::model::UserCredential;
use crate::credential_validation::domain::repository::UserCredentialRepository;
use crate::credential_validation::infrastructure::entity::sea_orm_active_enums::RolType;
use crate::credential_validation::infrastructure::entity::user;
use async_trait::async_trait;

pub struct SeaOrmUserCredentialRepository {
    pub db: DatabaseConnection,
}

#[async_trait]
impl UserCredentialRepository for SeaOrmUserCredentialRepository {
    async fn create_user(&self, user: UserCredential) -> Result<(), String> {
        let new_user = user::ActiveModel {
            name: Set(Some(user.name)),
            phone: Set(Some(user.phone)),
            address: Set(Some(user.address)),
            password: Set(Some(user.password)), 
            rol: Set(Some(RolType::Customer)), 
            ..Default::default()
        };

        new_user.insert(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_address(&self, address: &str) -> Result<Option<UserCredential>, String> {
        let result = user::Entity::find()
            .filter(user::Column::Address.eq(address))
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result.map(|u| UserCredential {
            id: Some(u.id_user),
            name: u.name.unwrap_or_default(),
            phone: u.phone.unwrap_or_default(),
            address: u.address.unwrap_or_default(),
            password: u.password.unwrap_or_default(),
        }))
    }
}
