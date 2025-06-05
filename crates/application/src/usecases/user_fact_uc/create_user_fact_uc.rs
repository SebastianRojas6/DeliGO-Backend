use crate::impl_mappers::user_mapper::UserMapper;
use crate::mappers::app_mapper::DTOMapper;
use crate::repositories::user_fact_repository_abstract::UserFactRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;
use crate::utils::error_handling_utils::ErrorHandlingUtils;
use crate::dtos::user::user_in_dto::UserCreateInDTO;
use crate::dtos::user::user_out_dto::UserOutDTO;
use domain::error::ApiError;

pub struct CreateUserFactUseCase<'a> {
    user: &'a UserCreateInDTO,
    repository: &'a dyn UserFactRepositoryAbstract,
}

impl<'a> CreateUserFactUseCase<'a> {
    pub fn new(user: &'a UserCreateInDTO, repository: &'a dyn UserFactRepositoryAbstract) -> Self {
        CreateUserFactUseCase { user, repository }
    }
}

#[async_trait::async_trait(?Send)]
impl<'a> AbstractUseCase<UserOutDTO> for CreateUserFactUseCase<'a> {
    async fn execute(&self) -> Result<UserOutDTO, ApiError> {
        let result = self.repository.create_user_fact(self.user.clone()).await;
        match result {
            Ok(user) => Ok(UserMapper::to_dto(user)),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot create user fact", Some(e))),
        }
    }
}
