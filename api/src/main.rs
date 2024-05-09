#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use log::info;
use rocket::serde::Deserialize;
use snafu::prelude::*;
use sqlx::sqlite::SqlitePool;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let rocket = rocket::build();
    let figment = rocket.figment();

    let config: Config = figment.extract().expect("config");
    let database_url: String = figment.extract_inner("database_url").expect("databaseUrl");

    info!("DATABASE_URL: {}", database_url);

    let pool = SqlitePool::connect(&database_url).await.context(SQLXSnafu)?;

    sqlx::migrate!().run(&pool).await.context(MigrationSnafu)?;
    
    rocket
        .mount("/", routes![index])
        .launch()
        .await
        .context(RocketSnafu)?;

    Ok(())
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    database_url: String,
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("There is a problem with the DB Initialization: {}", source))]
    SQLX { source: sqlx::Error },

    #[snafu(display("Failed to run migrations: {}", source))]
    Migration { source: sqlx::migrate::MigrateError },

    #[snafu(display("Rocket Error: {}", source))]
    Rocket { source: rocket::Error },
}
