#[macro_use]
extern crate rocket;

use log::info;
use snafu::prelude::*;
use sqlx::sqlite::SqlitePool;

mod models;
mod queries;
mod routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().context(DotenvSnafu)?;

    let rocket = rocket::build();
    let figment = rocket.figment();

    let database_url: String = figment.extract_inner("database_url").expect("databaseUrl");

    info!("DATABASE_URL: {}", database_url);

    let db_pool = SqlitePool::connect(&database_url)
        .await
        .context(SqlxSnafu)?;

    sqlx::migrate!()
        .run(&db_pool)
        .await
        .context(MigrationSnafu)?;

    rocket
        .manage(AppState { db_pool })
        .mount("/", routes![index])
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

    #[snafu(display("Failed to run migrations: {}", source))]
    Migration { source: sqlx::migrate::MigrateError },

    #[snafu(display("Rocket Error: {}", source))]
    Rocket { source: rocket::Error },
}
