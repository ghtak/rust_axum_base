use axum::extract::FromRef;

use crate::app_state::AppState;
use crate::entity::user::User;
use crate::{database, diag};

pub struct UserRepository {
    db_pool: database::DatabasePoolType,
}

impl UserRepository {
    pub(crate) async fn create(&self, user: User) -> diag::Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#" insert into "user"(name, email) values($1, $2) returning * "#,
        )
        .bind(user.name)
        .bind(user.email)
        .fetch_one(&self.db_pool)
        .await?;
        Ok(user)
    }

    pub(crate) async fn find_all(&self) -> diag::Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(r#"select * from "user""#)
            .fetch_all(&self.db_pool)
            .await?;
        Ok(users)
    }

    pub(crate) async fn find_by_id(&self, id: i32) -> diag::Result<User> {
        let user = sqlx::query_as::<_, User>(r#"select * from "user" where id = ($1) "#)
            .bind(id)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(user)
        //Err(AppError::Unknown("".to_owned()))
    }
}

impl FromRef<AppState> for UserRepository {
    fn from_ref(input: &AppState) -> Self {
        UserRepository {
            db_pool: input.db_pool.clone(),
        }
    }
}
