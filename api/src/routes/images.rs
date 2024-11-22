use crate::AppState;
use core_victorhqc_com::{
    aws::{
        image_size::{Error as ParseError, ImageSize},
        photo::{ByteStreamError, Error as AWSError},
    },
    models::photo::{db::Error as PhotoDbError, Photo},
    sqlx::Error as SqlxError,
};
use log::{debug, error};
use rocket::{
    http::{ContentType, Status},
    response::{stream::ByteStream, Responder},
    serde::json::serde_json,
    Response, State,
};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use snafu::prelude::*;
use std::io::Cursor;
use std::str::FromStr;

#[get("/images/<id>/<size>")]
pub async fn get_image(
    id: &str,
    size: &str,
    state: &State<AppState>,
) -> Result<(Status, (ContentType, ByteStream![Vec<u8>])), Error> {
    let s3 = &state.s3;
    let pool = &state.db_pool;
    let cache = &state.img_cache;
    let mut conn = pool.acquire().await.context(ConnectionSnafu)?;

    let img_size: ImageSize = ImageSize::from_str(size).context(SizeSnafu {
        size: size.to_string(),
    })?;

    debug!("id: {}", id);

    let photo = Photo::find_by_id(&mut conn, id).await.context(PhotoSnafu)?;

    // TODO: Ideally, this whole if-else block would be done by streaming bytes, rather than placing
    // the whole image in memory. At this time, every request clones the image in the cache and
    // sends it to the customer.
    let bytes = if let Some(bytes) = cache.get(&photo.id, &img_size) {
        debug!("Fetching from cache");

        bytes
    } else {
        debug!("Fetching from S3");

        let response = s3
            .download_from_aws_s3((&photo, &img_size))
            .await
            .context(GetAWSObjectSnafu)?;

        let data = response.body.collect().await.context(StreamSnafu)?;
        let bytes = data.into_bytes().to_vec();
        cache.save(&photo.id, &img_size, bytes.clone());

        bytes
    };

    Ok((
        Status::Ok,
        (
            ContentType::JPEG,
            ByteStream! {
                yield bytes
            },
        ),
    ))
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid size '{}': {}", size, source))]
    Size { size: String, source: ParseError },

    #[snafu(display("Failed to get AWS Object: {}", source))]
    GetAWSObject { source: AWSError },

    #[snafu(display("Failed to get connection: {}", source))]
    Connection { source: SqlxError },

    #[snafu(display("Failed to get photo by id: {}", source))]
    Photo { source: PhotoDbError },

    #[snafu(display("Failed to download photo: {}", source))]
    Stream { source: ByteStreamError },
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status: Status = match &self {
            Error::Size { .. } => Status::InternalServerError,
            Error::GetAWSObject { .. } => Status::NotFound,
            Error::Connection { .. } => Status::InternalServerError,
            Error::Photo { .. } => Status::InternalServerError,
            Error::Stream { .. } => Status::InternalServerError,
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
        let mut state = serializer.serialize_struct("Error", 2)?;

        match self {
            Error::Size { size, .. } => {
                state.serialize_field("kind", &format!("Invalid Size: {}", size))?;
                state.serialize_field("message", &self.to_string())?;
            }
            Error::GetAWSObject { source } => {
                state.serialize_field("kind", "Invalid Photo")?;
                state.serialize_field("message", "")?;

                error!("Failed to get AWS Object: {}", source);
            }
            Error::Connection { .. } => {
                state.serialize_field("kind", "Connection Error")?;
                state.serialize_field("message", &self.to_string())?;
            }
            Error::Photo { .. } => {
                state.serialize_field("kind", "Photo Error")?;
                state.serialize_field("message", &self.to_string())?;
            }
            Error::Stream { source } => {
                state.serialize_field("kind", "Stream Error")?;
                state.serialize_field("message", "")?;

                error!("Failed to stream image: {}", source);
            }
        };

        state.end()
    }
}
