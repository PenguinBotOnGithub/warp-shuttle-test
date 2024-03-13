use std::{convert::Infallible, sync::Arc};

use sea_orm::DatabaseConnection;
use warp::{filters::BoxedFilter, reply::Reply, Filter};

use crate::user::get_users_handler;

pub fn api_routes(
    db_filter: BoxedFilter<(Arc<DatabaseConnection>,)>,
) -> BoxedFilter<(impl Reply,)> {
    let iwak_route = warp::any()
        .and(warp::path("api").and(warp::path("iwak").and(warp::path::end()).map(iwak_handler)));

    let get_users = warp::get()
        .and(warp::path("api"))
        .and(warp::path("users"))
        .and(warp::path::end())
        .and(db_filter)
        .and_then(get_users_handler)
        .recover(|_e| async { Ok::<warp::reply::Json, Infallible>(warp::reply::json(&"lah")) });

    let routes = warp::any()
        .and(warp::path("api"))
        .and(warp::path::end())
        .map(|| "Hello from API!")
        .or(iwak_route)
        .or(get_users)
        .boxed();

    routes
}

fn iwak_handler() -> impl Reply {
    "Iwak 🐟🐟🐟!"
}
