use crate::modules::unsplash::{fetch_random_picture, Picture};
use rocket::serde::{json::Json, Serialize};

#[get("/unsplash/picture")]
pub async fn get_random_picture() -> Json<UnsplashPicture> {
    let picture = fetch_random_picture().await.unwrap();
    Json(UnsplashPicture { data: picture })
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UnsplashPicture {
    data: Picture,
}
