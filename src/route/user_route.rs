use std::collections::HashMap;

use axum::{extract::{Path, Query}, routing::{get, post}, Json, Router};
use axum_extra::extract::WithRejection;

use crate::{
    app_state::AppState,
    diag::{self, AppError},
    entity::{self, user::User},
    repository::{user_repository::UserRepository, Repository}, usecase::{get_user_usecase::GetUserUsecase, Usecase},
};

async fn create_user(
    Repository(user_repository): Repository<UserRepository>,
    Query(query): Query<HashMap<String, String>>,
) -> diag::Result<Json<User>> {
    let param_user = User::new(
        query.get("name").unwrap(), 
        query.get("email").unwrap());
    let user = user_repository.create(param_user).await?;
    Ok(Json(user))
}

async fn post_user(
    Repository(user_repository): Repository<UserRepository>,
    WithRejection(Json(user), _): WithRejection<Json<entity::user::User>, AppError>,
) -> diag::Result<Json<User>> {
    let user = user_repository.create(user).await?;
    Ok(Json(user))
}


async fn get_users(
    Repository(user_repository): Repository<UserRepository>,
) -> diag::Result<Json<Vec<User>>> {
    let users = user_repository.find_all().await?;
    Ok(Json(users))
}

async fn get_users_usecase(
    Usecase(user_usecase): Usecase<GetUserUsecase>,
) -> diag::Result<Json<Vec<User>>> {
    let users = user_usecase.get_all().await?;
    Ok(Json(users))
}

async fn get_user(
    Path(user_id): Path<i32>, 
    Usecase(user_usecase): Usecase<GetUserUsecase>
) -> diag::Result<Json<User>> {
    let user = user_usecase.find_by_id(user_id).await?;
    Ok(Json(user))
}

pub(crate) fn user_route() -> Router<AppState> {
    Router::new()
        .route("/user", post(post_user))
        .route("/users", get(get_users_usecase))
        .route("/user/:user_id", get(get_user))
}
