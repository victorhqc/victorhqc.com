use crate::models::photo::Photo;
use crate::AppState;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[get("/photos", format = "json")]
pub async fn get_all_photos(state: &State<AppState>) -> (Status, Json<Vec<Photo>>) {
    let pool = &state.db_pool;

    let photos = Photo::find_all(pool).await.unwrap();

    (Status::Ok, Json(photos))
}
