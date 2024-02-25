use std::sync::Arc;

use parking_lot::RwLock;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use shuttle_runtime::Error;
use warp::Filter;
use warp::Reply;

#[shuttle_runtime::main]
async fn warp(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:postgres@localhost:5432/db_shuttle_test"
    )]
    conn_str: String,
) -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let connect_opt = ConnectOptions::new(conn_str);
    let db = Arc::new(RwLock::new(
        Database::connect(connect_opt)
            .await
            .map_err(|e| Error::Database(e.to_string()))?,
    ));
    let db_filter = warp::any().map(move || Arc::clone(&db));

    let route = warp::any()
        .and(db_filter)
        .map(|db| "Hello from Warp with PostgreSQL DB;");
    Ok(route.boxed().into())
}
