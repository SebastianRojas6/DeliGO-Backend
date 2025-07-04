use async_trait::async_trait;
use super::model::UserCredential;

#[async_trait]
pub trait UserCredentialRepository: Send + Sync {
    async fn create_user(&self, user: UserCredential) -> Result<(), String>;
    async fn find_by_address(&self, address: &str) -> Result<Option<UserCredential>, String>;
}
