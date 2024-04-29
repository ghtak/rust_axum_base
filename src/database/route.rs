use std::collections::HashMap;

use axum::{extract::Query, routing::get, Json, Router};

use crate::{app_state::AppState, diag};

use super::{sample_user_repository::{User, UserRepository}, Repository};

async fn create_user(
    Repository(user_repository): Repository<UserRepository>,
    Query(query): Query<HashMap<String,String>>
) -> diag::Result<Json<User>>{
    let user = User{
        id: i32::default(),
        name: query.get("name").unwrap().to_owned(),
        email: query.get("email").unwrap().to_owned()
    };
    let user = user_repository.create(user).await?;
    Ok(Json(user))
}

async fn get_users(
    Repository(user_repository): Repository<UserRepository>,
) -> diag::Result<Json<Vec<User>>>
{
    let users = user_repository.find_all().await?;
    Ok(Json(users))
}


pub(crate) fn user_route() -> Router<AppState> {
    Router::new()
        .route("/user/create", get(create_user))
        .route("/user/list", get(get_users))
}
