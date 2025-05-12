use domain::user::UserEntity;
use crate::DTOs::user_dto::CreateUserDTOs;
use crate::repositories::user_fact_repository_abstract::UserFactRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;
use crate::utils::error_handling_utils::ErrorHandlingUtils;

pub struct CreateUserFactUseCase<'a> {
    user: &'a CreateUserDTOs,
    repository: &'a dyn UserFactRepositoryAbstract,
}

impl<'a> CreateUserFactUseCase<'a> {
    pub fn new(
        user: &'a CreateUserDTOs,
        repository: &'a dyn UserFactRepositoryAbstract,
    ) -> Self {
        CreateUserFactUseCase { user, repository }
    }
}

#[async_trait::async_trait(?Send)]
impl<'a> AbstractUseCase<UserEntity> for CreateUserFactUseCase<'a> {
    async fn execute(&self) -> Result<UserEntity, domain::error::ApiError> {
        let result = self.repository.create_user_fact(self.user.clone()).await;
        match result {
            Ok(user) => Ok(user),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot create user fact", Some(e))),
        }
    }
}