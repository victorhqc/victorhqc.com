use crate::AppState;
use core_victorhqc_com::models::{photo::Photo, tag::Tag};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[get("/photos", format = "json")]
pub async fn get_all_photos(state: &State<AppState>) -> (Status, Json<Vec<Photo>>) {
    let pool = &state.db_pool;
    let mut conn = pool.try_acquire().unwrap();

    let photos = Photo::find_all(&mut conn).await.unwrap();

    (Status::Ok, Json(photos))
}

#[get("/photos/<tag>", format = "json")]
pub async fn get_random_photo(tag: &str, state: &State<AppState>) -> (Status, Json<Vec<Photo>>) {
    let pool = &state.db_pool;
    let mut conn = pool.try_acquire().unwrap();

    let tag = Tag::find_by_name(&mut conn, tag).await.unwrap();
    let photos = Photo::find_by_tag_ids(&mut conn, &vec![tag.id])
        .await
        .unwrap()
        .into_iter()
        .map(|(_, photo)| photo)
        .collect();

    (Status::Ok, Json(photos))
}
