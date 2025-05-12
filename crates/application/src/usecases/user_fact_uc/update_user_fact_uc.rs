use domain::error::ApiError;
use domain::user::UserEntity;
use crate::DTOs::user_dto::UpdateUserDTOs;
use crate::repositories::user_fact_repository_abstract::UserFactRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;
use crate::utils::error_handling_utils::ErrorHandlingUtils;

pub struct UpdateUserFactUseCase<'a> {
    user: &'a UpdateUserDTOs,
    repository: &'a dyn UserFactRepositoryAbstract,
}

impl<'a> UpdateUserFactUseCase<'a> {
    pub fn new(
        user: &'a UpdateUserDTOs,
        repository: &'a dyn UserFactRepositoryAbstract,
    ) -> Self {
        UpdateUserFactUseCase { user, repository }
    }
}

#[async_trait::async_trait(?Send)]
impl<'a> AbstractUseCase<UserEntity> for UpdateUserFactUseCase<'a> {
    async fn execute(&self) -> Result<UserEntity, ApiError> {
        let result = self.repository.update_user_fact(self.user.clone()).await;
        match result {
            Ok(user) => Ok(user),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot update user fact", Some(e))),
        }
    }
}