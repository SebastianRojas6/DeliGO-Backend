use crate::repositories::user_fact_repository_abstract::UserFactRepositoryAbstract;
use crate::usecases::interfaces::AbstractUseCase;
use domain::user::UserEntity;

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
impl<'a> AbstractUseCase<UserEntity> for GetOneUserByIdFactUseCase<'a> {
    async fn execute(&self) -> Result<UserEntity, domain::error::ApiError> {
        let user = self.repository.get_one_user_by_id_fact(*self.user_fact_id).await;
        match user {
            Ok(user) => Ok(user),
            Err(e) => Err(crate::utils::error_handling_utils::ErrorHandlingUtils::application_error("Cannot get one user", Some(e))),
        }
    }
}
