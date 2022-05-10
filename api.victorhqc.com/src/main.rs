#[macro_use]
extern crate rocket;

#[macro_use]
extern crate dotenv_codegen;

mod handlers;
mod modules;

use dotenv::dotenv;
use handlers::unsplash;
use rocket::serde::{json::Json, Serialize};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let _rocket = rocket::build()
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
