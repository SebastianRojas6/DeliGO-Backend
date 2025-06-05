use domain::error::ApiError;
use crate::DTOs::user::user_out_dto::UserOutDTO;
use crate::impl_mappers::user_mapper::UserMapper;
use crate::mappers::app_mapper::DTOMapper;
use crate::repositories::user_fact_repository_abstract::UserFactRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;

pub struct GetAllUserFactsUseCase<'a> {
    repository: &'a dyn UserFactRepositoryAbstract,
}

impl <'a> GetAllUserFactsUseCase<'a> {
    pub fn new(repository: &'a dyn UserFactRepositoryAbstract) -> Self{ 
        GetAllUserFactsUseCase { repository }
    }
}

#[async_trait::async_trait(?Send)]
impl<'a> AbstractUseCase<Vec<UserOutDTO>> for GetAllUserFactsUseCase<'a> {
    async fn execute(&self) -> Result<Vec<UserOutDTO>, ApiError> {
        let user_facts = self.repository.get_all_user_facts().await;
        match user_facts { 
            Ok(facts) => Ok(UserMapper::to_dtos(facts)),
            Err(e) => Err(crate::utils::error_handling_utils::ErrorHandlingUtils::application_error("Cannot get all cat facts", Some(e))),
        }
    }
}