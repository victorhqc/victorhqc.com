#[macro_use]
extern crate rocket;

use crate::graphql::context::Context;
use crate::graphql::loaders::AppLoader;
use crate::graphql::routes::{graphql_playground, graphql_query, graphql_request};
use crate::graphql::{graph::RootSchema, queries::RootQuery, sdl_gen};
use async_graphql::{dataloader::DataLoader, EmptyMutation, EmptySubscription, Schema};
use log::info;
use rocket::tokio::spawn;
use snafu::prelude::*;
use sqlx::sqlite::SqlitePool;
use core_victorhqc_com::db::{migrate, Error as DBError};

mod graphql;
mod routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().context(DotenvSnafu)?;
    pretty_env_logger::init();

    let rocket = rocket::build();
    let figment = rocket.figment();

    let database_url: String = figment.extract_inner("database_url").expect("databaseUrl");

    info!("DATABASE_URL: {}", database_url);

    let db_pool = SqlitePool::connect(&database_url)
        .await
        .context(SqlxSnafu)?;

    #[cfg(debug_assertions)]
    {
        migrate(&db_pool).await.context(MigrationSnafu)?;
    }

    let context = Context::default(db_pool.clone());
    let loader = AppLoader::default(db_pool.clone());

    let schema: RootSchema = Schema::build(RootQuery::default(), EmptyMutation, EmptySubscription)
        .data(context)
        .data(DataLoader::new(loader, spawn))
        .finish();

    #[cfg(debug_assertions)]
    {
        sdl_gen(&schema).unwrap();
    }

    rocket
        .manage(schema)
        .manage(AppState { db_pool })
        .mount(
            "/",
            routes![index, graphql_query, graphql_request, graphql_playground],
        )
        .mount("/v1", routes![routes::photos::get_all_photos])
        .launch()
        .await
        .context(RocketSnafu)?;

    Ok(())
}

struct AppState {
    db_pool: SqlitePool,
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Failed to load configuration: {}", source))]
    Dotenv { source: dotenvy::Error },

    #[snafu(display("There is a problem with the DB Initialization: {}", source))]
    Sqlx { source: sqlx::Error },

    #[cfg(debug_assertions)]
    #[snafu(display("{}", source))]
    Migration { source: DBError },

    #[snafu(display("Rocket Error: {}", source))]
    Rocket { source: rocket::Error },
}
