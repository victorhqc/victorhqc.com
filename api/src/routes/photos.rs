use crate::models::{ExifMeta, Photo};
use crate::db::photos;
use crate::AppState;
use rocket::http::Status;
// use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

#[get("/photos", format = "json")]
pub async fn get_all_photos(state: &State<AppState>) -> (Status, Json<Vec<Photo>>) {
    let pool = &state.db_pool;

    let photos = photos::get_all_photos(pool).await.unwrap();

    (Status::Ok, Json(photos))
}
