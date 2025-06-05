use crate::impl_mappers::user_mapper::UserMapper;
use crate::mappers::app_mapper::DTOMapper;
use crate::repositories::user_fact_repository_abstract::UserFactRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;
use crate::utils::error_handling_utils::ErrorHandlingUtils;
use crate::dtos::user::user_in_dto::UserUpdateInDTO;
use crate::dtos::user::user_out_dto::UserOutDTO;
use domain::error::ApiError;

pub struct UpdateUserFactUseCase<'a> {
    user_id: i32,
    user: &'a UserUpdateInDTO,
    repository: &'a dyn UserFactRepositoryAbstract,
}

impl<'a> UpdateUserFactUseCase<'a> {
    pub fn new(user_id: i32, user: &'a UserUpdateInDTO, repository: &'a dyn UserFactRepositoryAbstract) -> Self {
        UpdateUserFactUseCase { user_id, user, repository }
    }
}

#[async_trait::async_trait(?Send)]
impl<'a> AbstractUseCase<UserOutDTO> for UpdateUserFactUseCase<'a> {
    async fn execute(&self) -> Result<UserOutDTO, ApiError> {
        let result = self.repository.update_user_fact(self.user_id, self.user.clone()).await;
        match result {
            Ok(user) => Ok(UserMapper::to_dto(user)),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot update user fact", Some(e))),
        }
    }
}
