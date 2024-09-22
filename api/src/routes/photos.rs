use crate::models::{ExifMeta, Photo};
use crate::queries::photos;
use crate::AppState;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

#[get("/photos", format = "json")]
pub async fn get_all_photos(state: &State<AppState>) -> status::Accepted<Json<Vec<(Photo, ExifMeta)>>> {
    let pool = &state.db_pool;

    let photos = photos::get_all_photos(pool).await.unwrap();

    status::Accepted(Json(photos))
}
