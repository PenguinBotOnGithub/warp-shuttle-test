use std::sync::Arc;

use entity::user::{self};
use sea_orm::{entity::*, DatabaseConnection, FromQueryResult, QuerySelect};
use serde::{Deserialize, Serialize};
use warp::{
    reject::{self},
    reply::Reply,
};

use crate::error::{ClientError, InternalError};
use crate::GenericResponse;

#[derive(Debug, Deserialize)]
pub struct UserRegisterModel {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserLoginModel {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, FromQueryResult)]
pub struct UserPublicModel {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub verified: bool,
}

pub async fn get_users_handler(db: Arc<DatabaseConnection>) -> Result<impl Reply, warp::Rejection> {
    let res = user::Entity::find()
        .select_only()
        .columns([
            user::Column::Id,
            user::Column::Email,
            user::Column::Username,
            user::Column::Verified,
        ])
        .into_model::<UserPublicModel>()
        .all(&*db)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    Ok(warp::reply::json(&GenericResponse::new(
        "success",
        Some(res),
    )))
}

pub async fn get_single_user_handler(
    id: u32,
    db: Arc<DatabaseConnection>,
) -> Result<impl Reply, warp::Rejection> {
    let res = user::Entity::find_by_id(id)
        .into_model::<UserPublicModel>()
        .one(&*db)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    match res {
        Some(v) => Ok(warp::reply::json(&GenericResponse::new("success", v))),
        None => Err(reject::custom(ClientError::NotFound)),
    }
}
