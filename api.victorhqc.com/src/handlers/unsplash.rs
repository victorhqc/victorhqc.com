use rocket::serde::{json::Json, Serialize};

#[get("/unsplash/picture")]
pub async fn get_random_picture() -> Json<UnsplashPicture> {
    Json(UnsplashPicture { url: "wip".into() })
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UnsplashPicture {
    url: String,
}
