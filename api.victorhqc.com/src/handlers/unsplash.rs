use crate::modules::unsplash::{
    fetch_random_picture, Picture, PicturesCache, UnsplashPicturesError,
};
use async_mutex::Mutex;
use rocket::{
    http::Status,
    response,
    response::Responder,
    serde::{json::Json, Serialize},
    Request, State,
};
use snafu::prelude::*;
use std::sync::Arc;

#[get("/unsplash/picture?<query>&<orientation>")]
pub async fn get_random_picture(
    raw_state: &State<Arc<Mutex<PicturesCache>>>,
    query: &str,
    orientation: &str,
) -> Result<Json<UnsplashPicture>> {
    let mut state = raw_state.lock().await;

    let q = match query {
        "" => "wallpaper",
        q => q,
    };

    let o = match orientation {
        "" => "landscape",
        o => o,
    };

    let (should_fetch, _) = state.should_fetch(q, o);
    if should_fetch {
        let picture = fetch_random_picture(q, o)
            .await
            .context(PictureIssueSnafu)?;
        state.set_random_picture(picture.clone(), q, o);

        Ok(Json(UnsplashPicture { picture: picture }))
    } else {
        let picture = state.get_last_random(q, o).unwrap();

        Ok(Json(UnsplashPicture {
            picture: picture.clone(),
        }))
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UnsplashPicture {
    picture: Picture,
}

pub type Result<T, E = UnsplashHandlerError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum UnsplashHandlerError {
    #[snafu(display("Failed to get the picture {}", source))]
    PictureIssue { source: UnsplashPicturesError },
}

impl<'r, 'o: 'r> Responder<'r, 'o> for UnsplashHandlerError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        println!("Something went wrong! {:?}", &self);

        match self {
            _ => Status::InternalServerError.respond_to(req),
        }
    }
}
