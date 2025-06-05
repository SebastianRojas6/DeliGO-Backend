use crate::impl_mappers::user_mapper::UserMapper;
use crate::mappers::app_mapper::DTOMapper;
use crate::repositories::user_fact_repository_abstract::UserFactRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;
use crate::DTOs::user::user_out_dto::UserOutDTO;
use domain::error::ApiError;

pub struct GetOneUserByIdFactUseCase<'a> {
    user_fact_id: &'a i32,
    repository: &'a dyn UserFactRepositoryAbstract,
}

impl<'a> GetOneUserByIdFactUseCase<'a> {
    pub fn new(user_fact_id: &'a i32, repository: &'a dyn UserFactRepositoryAbstract) -> Self {
        GetOneUserByIdFactUseCase { user_fact_id, repository }
    }
}

#[async_trait::async_trait(?Send)]
impl<'a> AbstractUseCase<UserOutDTO> for GetOneUserByIdFactUseCase<'a> {
    async fn execute(&self) -> Result<UserOutDTO, ApiError> {
        let user = self.repository.get_one_user_by_id_fact(*self.user_fact_id).await;
        match user {
            Ok(user) => Ok(UserMapper::to_dto(user)),
            Err(e) => Err(crate::utils::error_handling_utils::ErrorHandlingUtils::application_error("Cannot get one user", Some(e))),
        }
    }
}
