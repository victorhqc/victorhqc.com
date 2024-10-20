use crate::AppState;
use core_victorhqc_com::{aws::photo::ImageSize, models::photo::Photo};
use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::serde::json::serde_json;
use rocket::{http::Status, serde::json::Json, Response, State};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use snafu::prelude::*;
use std::io::Cursor;
use std::str::FromStr;
use strum::ParseError;

#[get("/image/<id>/<size>")]
pub async fn get_image(
    id: String,
    size: String,
    state: &State<AppState>,
) -> Result<(Status, Json<Vec<Photo>>), Error> {
    let s3 = &state.s3;
    let img_size: ImageSize =
        ImageSize::from_str(&size).context(SizeSnafu { size: size.clone() })?;

    // let pool = &state.db_pool;
    // let mut conn = pool.try_acquire().unwrap();
    //
    // let photos = Photo::find_all(&mut conn).await.unwrap();
    //
    // (Status::Ok, Json(photos))

    todo!()
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid size '{}': {}", size, source))]
    Size { size: String, source: ParseError },
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status: Status = match &self {
            Error::Size { .. } => Status::InternalServerError,
        };

        let serialized = serde_json::to_string(&self).unwrap();

        Response::build()
            .status(status)
            .sized_body(serialized.len(), Cursor::new(serialized))
            .header(ContentType::JSON)
            .ok()
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let kind: String = match self {
            Error::Size { size, .. } => format!("Invalid Size: {}", size),
        };

        let mut state = serializer.serialize_struct("Error", 2)?;
        state.serialize_field("kind", &kind)?;
        state.serialize_field("message", &self.to_string())?;

        state.end()
    }
}
