use std::collections::HashMap;

use axum::{extract::Query, routing::get, Json, Router};

use crate::{
    app_state::AppState,
    diag,
    entity::user::User,
    repository::{user_repository::UserRepository, Repository}, usecase::{user_usecase::UserUsecase, Usecase},
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

async fn get_users(
    Repository(user_repository): Repository<UserRepository>,
) -> diag::Result<Json<Vec<User>>> {
    let users = user_repository.find_all().await?;
    Ok(Json(users))
}

async fn get_users_usecase(
    Usecase(user_usecase): Usecase<UserUsecase>,
) -> diag::Result<Json<Vec<User>>> {
    let users = user_usecase.get_users().await?;
    Ok(Json(users))
}

pub(crate) fn user_route() -> Router<AppState> {
    Router::new()
        .route("/user/create", get(create_user))
        .route("/user/list", get(get_users))
        .route("/user/list_u", get(get_users_usecase))
}
