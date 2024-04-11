use std::{convert::Infallible, error::Error};
use tracing::error;

use warp::http::StatusCode;
use warp::reject;

use crate::GenericResponse;

#[derive(Debug)]
pub enum ClientError {
    Conflict,
    Unauthorized,
    NotFound,
}

#[derive(Debug)]
pub enum InternalError {
    DatabaseError(String),
    ArgonError(String),
}

impl reject::Reject for InternalError {}
impl reject::Reject for ClientError {}

pub async fn handle_rejection(error: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let (data, code) = if error.is_not_found() {
        ("Not Found".to_owned(), StatusCode::NOT_FOUND)
    } else if let Some(e) = error.find::<warp::reject::InvalidHeader>() {
        (e.to_string(), StatusCode::BAD_REQUEST)
    } else if let Some(e) = error.find::<warp::body::BodyDeserializeError>() {
        (
            match e.source() {
                Some(e) => e.to_string(),
                None => "Bad Request".to_owned(),
            },
            StatusCode::BAD_REQUEST,
        )
    } else if let Some(e) = error.find::<ClientError>() {
        match e {
            ClientError::Conflict => ("Conflict".to_owned(), StatusCode::CONFLICT),
            ClientError::Unauthorized => ("Unauthorized".to_owned(), StatusCode::UNAUTHORIZED),
            ClientError::NotFound => ("Not Found".to_owned(), StatusCode::NOT_FOUND),
        }
    } else if let Some(e) = error.find::<InternalError>() {
        (
            match e {
                InternalError::DatabaseError(e) => {
                    error!("Database error: {}", e);
                    "Internal Server Error".to_owned()
                }
                InternalError::ArgonError(e) => {
                    error!("Argon error: {}", e);
                    "Internal Server Error".to_owned()
                }
            },
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    } else if let Some(e) = error.find::<warp::reject::MethodNotAllowed>() {
        (
            match e.source() {
                Some(e) => e.to_string(),
                None => "Method Not Allowed | Unhandled Endpoint".to_owned(),
            },
            StatusCode::METHOD_NOT_ALLOWED,
        )
    } else {
        error!("Unhandled rejection, please view logs for more details");
        ("Unhandled Rejection!!!".to_owned(), StatusCode::IM_A_TEAPOT)
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&GenericResponse::new("error", Some(data))),
        code,
    ))
}
