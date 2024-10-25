#[macro_use]
extern crate rocket;

use crate::cache::image_cache::ImageCache;
use crate::graphql::{
    context::Context,
    graph::RootSchema,
    loaders::AppLoader,
    queries::RootQuery,
    routes::{graphql_playground, graphql_query, graphql_request},
    sdl_gen,
};
use async_graphql::{dataloader::DataLoader, EmptyMutation, EmptySubscription, Schema};
use core_victorhqc_com::{
    aws::S3,
    db::{get_pool, migrate, Error as DBError},
    sqlx::sqlite::SqlitePool,
};
use log::info;
use rocket::tokio::spawn;
use snafu::prelude::*;

mod bootstrap;
mod cache;
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
    let bucket_name =
        std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME env variable is missing");
    let cached_tags: String = figment
        .extract_inner("cached_photo_tags")
        .unwrap_or("".to_string());

    let cached_tags: Vec<String> = cached_tags
        .split(',')
        .map(|t| t.trim().to_string())
        .collect();

    debug!("Cached Tags: {:?}", cached_tags);

    let s3 = S3::new(&bucket_name).await;

    info!("DATABASE_URL: {}", database_url);
    let db_pool = get_pool(&database_url).await.context(PoolSnafu)?;

    #[cfg(debug_assertions)]
    {
        migrate(&db_pool).await.context(MigrationSnafu)?;
    }

    let context = Context::default(db_pool.clone());
    let loader = AppLoader::default(db_pool.clone());
    let img_cache = ImageCache::default();

    let mut state = AppState {
        db_pool,
        s3,
        img_cache,
    };

    if cached_tags.len() > 0 {
        state = bootstrap::prepare_images(state, cached_tags).await.unwrap();
    }

    let schema: RootSchema = Schema::build(RootQuery::default(), EmptyMutation, EmptySubscription)
        .data(context)
        .data(DataLoader::new(loader, spawn))
        // .limit_depth(4)
        .finish();

    #[cfg(debug_assertions)]
    {
        sdl_gen(&schema).unwrap();
    }

    rocket
        .manage(schema)
        .manage(state)
        .mount(
            "/",
            routes![index, graphql_query, graphql_request, graphql_playground],
        )
        .mount(
            "/v1",
            routes![routes::photos::get_all_photos, routes::images::get_image],
        )
        .launch()
        .await
        .context(RocketSnafu)?;

    Ok(())
}

struct AppState {
    db_pool: SqlitePool,
    img_cache: ImageCache,
    s3: S3,
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Failed to load configuration: {}", source))]
    Dotenv { source: dotenvy::Error },

    #[cfg(debug_assertions)]
    #[snafu(display("{}", source))]
    Migration { source: DBError },

    #[snafu(display("{}", source))]
    Pool { source: DBError },

    #[snafu(display("Rocket Error: {}", source))]
    Rocket { source: rocket::Error },
}
