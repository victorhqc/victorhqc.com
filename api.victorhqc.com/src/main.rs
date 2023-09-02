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
use std::env;
use std::sync::Arc;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let image_cache_time: u64 = env::var("UNSPLASH_IMAGE_CACHE_IN_MINS")
        .expect("UNSPLASH_IMAGE_CACHE_IN_MINS is not defined")
        .to_string()
        .parse::<u64>()
        .expect("UNSPLASH_IMAGE_CACHE_IN_MINS is not a valid number");

    let image_cache_amount: usize = env::var("UNSPLASH_IMAGE_CACHE_AMOUNT")
        .unwrap_or(String::from("5"))
        .to_string()
        .parse::<usize>()
        .expect("UNSPLASH_IMAGE_CACHE_AMOUNT is not a valid number");

    let state = Arc::new(Mutex::new(PicturesCache::new(
        image_cache_amount,
        image_cache_time,
    )));

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
