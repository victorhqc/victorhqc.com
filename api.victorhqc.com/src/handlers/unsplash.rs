use crate::modules::unsplash::{fetch_random_picture, Picture, PicturesState};
use async_mutex::Mutex;
use rocket::{
    serde::{json::Json, Serialize},
    State,
};
use std::sync::Arc;

#[get("/unsplash/picture")]
pub async fn get_random_picture(
    raw_state: &State<Arc<Mutex<PicturesState>>>,
) -> Json<UnsplashPicture> {
    let mut state = raw_state.lock().await;

    if state.should_fetch() {
        let picture = fetch_random_picture().await.unwrap();
        state.set_random_picture(picture.clone());

        Json(UnsplashPicture { data: picture })
    } else {
        let picture = state.get_last_random().unwrap();

        Json(UnsplashPicture {
            data: picture.clone(),
        })
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UnsplashPicture {
    data: Picture,
}
