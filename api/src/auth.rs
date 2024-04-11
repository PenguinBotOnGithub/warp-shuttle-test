use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use entity::user;
use sea_orm::{entity::*, DatabaseConnection, QueryFilter, QuerySelect};
use warp::{reject, reply::Reply};

use crate::{
    error::{ClientError, InternalError},
    user::{UserLoginModel, UserRegisterModel},
    GenericResponse,
};

pub async fn register_user_handler(
    payload: UserRegisterModel,
    db: Arc<DatabaseConnection>,
) -> Result<impl Reply, warp::Rejection> {
    let already_exists: Option<(String,)> = user::Entity::find()
        .select_only()
        .column(user::Column::Email)
        .filter(user::Column::Email.eq(&payload.email))
        .into_tuple()
        .one(&*db)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let Some(_) = already_exists {
        reject::custom(ClientError::Conflict);
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_pass = Argon2::default()
        .hash_password(&payload.password.as_bytes(), &salt)
        .map_err(|e| reject::custom(InternalError::ArgonError(e.to_string())))
        .map(|hash| hash.to_string())?;

    let user = entity::user::ActiveModel {
        id: NotSet,
        email: Set(payload.email.clone()),
        username: Set(payload.username.clone()),
        password: Set(hashed_pass),
        verified: Set(false),
        created_at: NotSet,
    };
    let new_user = user
        .insert(&*db)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    Ok(warp::reply::json(&new_user))
}

pub async fn login_user_handler(
    payload: UserLoginModel,
    db: Arc<DatabaseConnection>,
) -> Result<impl Reply, warp::Rejection> {
    let user: Option<(String,)> = user::Entity::find()
        .select_only()
        .column(user::Column::Password)
        .filter(user::Column::Username.like(&payload.username))
        .into_tuple()
        .one(&*db)
        .await
        .map_err(|e| reject::custom(InternalError::DatabaseError(e.to_string())))?;

    if let None = user {
        reject::custom(ClientError::NotFound);
    }

    match Argon2::default().verify_password(
        &payload.password.as_bytes(),
        &PasswordHash::new(&user.unwrap().0)
            .map_err(|e| reject::custom(InternalError::ArgonError(e.to_string())))?,
    ) {
        Ok(_) => Ok(warp::reply::json(&GenericResponse::new(
            "success",
            Some("Authenticated"),
        ))),
        Err(_e) => Ok(warp::reply::json(&GenericResponse::new(
            "error",
            Some("Incorrect Password"),
        ))),
    }
}

// pub fn with_auth()
