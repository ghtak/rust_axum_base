use axum::extract::FromRef;

use crate::{app_state::AppState, diag, entity::user::User, repository::user_repository::UserRepository};


pub(crate) struct UserUsecase{
    user_repository: UserRepository
}

impl UserUsecase{
    pub(crate) async fn get_users(&self) -> diag::Result<Vec<User>> {
        self.user_repository.find_all().await
    }
}

impl FromRef<AppState> for UserUsecase {
    fn from_ref(input: &AppState) -> Self {
        UserUsecase {
            user_repository: UserRepository::from_ref(input),
        }
    }
}
