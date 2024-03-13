use std::sync::Arc;

use entity::user;
use sea_orm::{entity::*, DatabaseConnection};
use warp::{reject::reject, reply::Reply};

pub async fn get_users_handler(db: Arc<DatabaseConnection>) -> Result<impl Reply, warp::Rejection> {
    let res = user::Entity::find()
        .all(&*db)
        .await
        .map_err(|_e| reject())?;
    Ok(warp::reply::json(&res))
}
