use std::sync::Arc;

use anyhow::Error as anyError;
use api::error::handle_rejection;
use api::routes::api_routes;
use migration::MigratorTrait;
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
    let db = Database::connect(connect_opt)
        .await
        .map_err(|e| Error::Database(e.to_string()))?;
    migration::Migrator::up(&db, None)
        .await
        .map_err(|e| Error::Custom(anyError::msg(format!("Error running migrations: {e}"))))?;

    let db = Arc::new(db);
    let db_filter = warp::any().map(move || Arc::clone(&db)).boxed();

    let hello = warp::any()
        .and(warp::path::end())
        .then(|| async { "Hello, Wold!" });

    let routes = hello.or(api_routes(db_filter)).recover(handle_rejection);

    Ok(routes.boxed().into())
}
