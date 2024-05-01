use axum::extract::FromRef;

use crate::{app_state::AppState, diag, entity::user::User, repository::user_repository::UserRepository};


pub(crate) struct GetUserUsecase{
    user_repository: UserRepository
}

impl GetUserUsecase{
    pub(crate) async fn get_all(&self) -> diag::Result<Vec<User>> {
        self.user_repository.find_all().await
    }

    pub(crate) async fn find_by_id(&self, id: i32) -> diag::Result<User> {
        self.user_repository.find_by_id(id).await
    }
}

impl FromRef<AppState> for GetUserUsecase {
    fn from_ref(input: &AppState) -> Self {
        GetUserUsecase {
            user_repository: UserRepository::from_ref(input),
        }
    }
}
