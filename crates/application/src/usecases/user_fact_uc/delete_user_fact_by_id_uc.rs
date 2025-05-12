use crate::usecases::interfaces::AbstractUseCase;
use crate::repositories::user_fact_repository_abstract::UserFactRepositoryAbstract;
pub struct DeleteUserFactByIdUseCase<'a> {
    user_fact_id: &'a i32,
    repository: &'a dyn UserFactRepositoryAbstract,
}

impl<'a> DeleteUserFactByIdUseCase<'a> {
    pub fn new(
        user_fact_id: &'a i32,
        repository: &'a dyn UserFactRepositoryAbstract,
    ) -> Self {
        DeleteUserFactByIdUseCase { user_fact_id, repository }
    }
}

#[async_trait::async_trait(?Send)]
impl<'a> AbstractUseCase<()> for DeleteUserFactByIdUseCase<'a> {
    async fn execute(&self) -> Result<(), domain::error::ApiError> {
        let result = self
            .repository
            .delete_user_by_id_fact(*self.user_fact_id)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(crate::utils::error_handling_utils::ErrorHandlingUtils::application_error(
                "Cannot delete user fact",
                Some(e),
            )),
        }
    }
}