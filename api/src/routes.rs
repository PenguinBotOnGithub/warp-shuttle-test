use std::sync::Arc;

use sea_orm::DatabaseConnection;
use tracing::info;
use warp::{filters::BoxedFilter, reject::Rejection, reply::Reply, Filter};

use crate::{
    auth::{login_user_handler, register_user_handler},
    user::{get_single_user_handler, get_users_handler},
    with_json, GenericResponse,
};

pub fn api_routes(
    with_db: BoxedFilter<(Arc<DatabaseConnection>,)>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // Endpoints
    let api_endpoint = warp::path("api");
    let auth_endpoint = api_endpoint.and(warp::path("auth"));
    let users_endpoint = api_endpoint.and(warp::path("users"));

    let iwak_route = warp::any()
        .and(api_endpoint)
        .and(warp::path::end())
        .then(iwak_handler);

    let get_users = warp::get()
        .and(users_endpoint)
        .and(warp::path::end())
        .and(with_db.clone())
        .and_then(get_users_handler);

    let get_single_user = warp::get()
        .and(users_endpoint)
        .and(warp::path::param::<u32>())
        .and(warp::path::end())
        .and(with_db.clone())
        .and_then(get_single_user_handler);

    let create_user = warp::post()
        .and(auth_endpoint)
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(with_json())
        .and(with_db.clone())
        .and_then(register_user_handler);

    let login_user = warp::post()
        .and(auth_endpoint)
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(with_json())
        .and(with_db.clone())
        .and_then(login_user_handler);

    let routes = iwak_route
        .or(create_user)
        .or(get_users)
        .or(get_single_user)
        .or(login_user);

    routes
}

async fn iwak_handler() -> impl Reply {
    info!("Iwak 🐟🐟🐟!");
    warp::reply::json(&GenericResponse::new("success", Some("Iwak 🐟🐟🐟!")))
}
