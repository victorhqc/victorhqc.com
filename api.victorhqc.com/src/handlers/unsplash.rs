use crate::modules::unsplash::pictures::fetch_random_picture;
use rocket::serde::{json::Json, Serialize};

#[get("/unsplash/picture")]
pub async fn get_random_picture() -> Json<UnsplashPicture> {
    fetch_random_picture().await.unwrap();
    Json(UnsplashPicture { url: "wip".into() })
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UnsplashPicture {
    url: String,
}
