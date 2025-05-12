use domain::user::UserEntity;
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
impl<'a> AbstractUseCase<Vec<UserEntity>> for GetAllUserFactsUseCase<'a> {
    async fn execute(&self) -> Result<Vec<UserEntity>, domain::error::ApiError> {
        let user_facts = self.repository.get_all_user_facts().await;
        match user_facts { 
            Ok(facts) => Ok(facts),
            Err(e) => Err(crate::utils::error_handling_utils::ErrorHandlingUtils::application_error("Cannot get all cat facts", Some(e))),
        }
    }
}