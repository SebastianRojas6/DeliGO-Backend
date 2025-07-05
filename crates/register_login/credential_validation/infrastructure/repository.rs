use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};

use crate::credential_validation::domain::model::UserCredential;
use crate::credential_validation::domain::repository::UserCredentialRepository;
use shared::entity::sea_orm_active_enums::RolType;
use shared::entity::user;

use async_trait::async_trait;

pub struct SeaOrmUserCredentialRepository {
    pub db: DatabaseConnection,
}

#[async_trait]
impl UserCredentialRepository for SeaOrmUserCredentialRepository {
    async fn create_user(&self, user: UserCredential) -> Result<(), String> {
        let new_user = user::ActiveModel {
            name: Set(Some(user.name)),
            phone: Set(user.phone),
            email: Set(user.email),
            address: Set(user.address),
            password: Set(user.password),
            rol: Set(RolType::Customer),
            ..Default::default()
        };

        new_user.insert(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserCredential>, String> {
        let result = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result.map(|u| UserCredential {
            id: Some(u.id_user),
            name: u.name.unwrap_or_default(),
            phone: u.phone,
            email: u.email,
            password: u.password,
            address: u.address,
        }))
    }
}
