#[macro_use]
extern crate rocket;

#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate log;

mod handlers;
mod modules;

use async_mutex::Mutex;
use dotenv::dotenv;
use handlers::unsplash;
use modules::unsplash::PicturesState;
use rocket::serde::{json::Json, Serialize};
use std::sync::Arc;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let state = Arc::new(Mutex::new(PicturesState::new()));

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
