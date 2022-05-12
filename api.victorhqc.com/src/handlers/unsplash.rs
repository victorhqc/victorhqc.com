use crate::modules::unsplash::{fetch_random_picture, Picture, PicturesCache};
use async_mutex::Mutex;
use rocket::{
    serde::{json::Json, Serialize},
    State,
};
use std::sync::Arc;

#[get("/unsplash/picture?<query>&<orientation>")]
pub async fn get_random_picture(
    raw_state: &State<Arc<Mutex<PicturesCache>>>,
    query: &str,
    orientation: &str,
) -> Json<UnsplashPicture> {
    let mut state = raw_state.lock().await;

    let q = match query {
        "" => "wallpaper",
        q => q,
    };

    let o = match orientation {
        "" => "landscape",
        o => o,
    };

    if state.should_fetch(q, o) {
        let picture = fetch_random_picture(q, o).await.unwrap();
        state.set_random_picture(picture.clone(), q, o);

        Json(UnsplashPicture { picture: picture })
    } else {
        let picture = state.get_last_random(q, o).unwrap();

        Json(UnsplashPicture {
            picture: picture.clone(),
        })
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UnsplashPicture {
    picture: Picture,
}
