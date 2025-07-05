use async_trait::async_trait;
use super::model::UserCredential;

#[async_trait]
pub trait UserCredentialRepository: Send + Sync {
    async fn create_user(&self, user: UserCredential) -> Result<(), String>;
    async fn find_by_email(&self, email: &str) -> Result<Option<UserCredential>, String>;
}
