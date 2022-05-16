#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

mod handlers;
mod modules;

use async_mutex::Mutex;
use dotenv::dotenv;
use handlers::unsplash;
use modules::unsplash::PicturesCache;
use rocket::serde::{json::Json, Serialize};
use std::sync::Arc;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let image_cache_time: String = option_env!("UNSPLASH_IMAGE_CACHE_IN_MINS")
        .expect("UNSPLASH_IMAGE_CACHE_IN_MINS is not defined")
        .to_string();
    let image_cache_time = image_cache_time
        .parse::<u64>()
        .expect("UNSPLASH_IMAGE_CACHE_IN_MINS is not a valid number");

    let state = Arc::new(Mutex::new(PicturesCache::new(30, image_cache_time)));

    let _rocket = rocket::build()
        .manage(state)
        .mount("/", routes![index])
        .mount("/v1/", routes![unsplash::get_random_picture])
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
async fn index() -> Json<Hello> {
    Json(Hello {
        message: "Hello, this is a simple API for my website.".into(),
    })
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Hello {
    message: String,
}
