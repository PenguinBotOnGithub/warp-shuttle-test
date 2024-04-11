use serde::{de::DeserializeOwned, Deserialize, Serialize};
use warp::Filter;

pub mod auth;
pub mod error;
pub mod routes;
pub mod user;

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericResponse<T> {
    message: String,
    data: Option<T>,
}

impl<T> GenericResponse<T> {
    pub fn new(message: impl Into<String>, data: T) -> GenericResponse<T> {
        GenericResponse {
            message: message.into(),
            data: Some(data),
        }
    }
}

pub fn with_json<J>() -> impl Filter<Extract = (J,), Error = warp::Rejection> + Clone
where
    J: DeserializeOwned,
    J: Send + Sync,
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
