use crate::AppState;
use core_victorhqc_com::models::photo::Photo;
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
