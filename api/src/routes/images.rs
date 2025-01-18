use crate::cache::image_cache::Error as CacheError;
use crate::AppState;
use core_victorhqc_com::{
    aws::{
        image_size::{Error as ParseError, ImageSize},
        photo::ByteStreamError,
    },
    models::photo::{db::Error as PhotoDbError, Photo},
    sqlx::Error as SqlxError,
};
use log::{debug, error};
use rocket::{
    http::{hyper::body::Bytes, ContentType, Status},
    response::{stream::ByteStream, Responder},
    serde::json::serde_json,
    Response, State,
};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use snafu::prelude::*;
use std::io::Cursor;
use std::str::FromStr;

#[get("/images/<size>/<id>")]
pub async fn get_image(
    size: &str,
    id: &str,
    state: &State<AppState>,
) -> Result<(Status, (ContentType, ByteStream![Bytes])), Error> {
    let pool = &state.db_pool;
    let cache = &state.img_cache;
    let mut conn = pool.acquire().await.context(ConnectionSnafu)?;

    let img_size: ImageSize = ImageSize::from_str(size).context(SizeSnafu {
        size: size.to_string(),
    })?;

    debug!("id: {}", id);

    let photo = Photo::find_by_id(&mut conn, id).await.context(PhotoSnafu)?;

    // TODO: Ideally, the `.get()` method would not clone what's in memory, and simply stream the
    // image from memory to the response of the request.
    // let mut stream = if let Some(bytes) = cache.get(&photo.id, &img_size) {
    //     debug!("Fetching from cache");

    //     AWSByteStream::from(bytes)
    // } else {
    //     debug!("Fetching from S3");

    //     let response = cache
    //         .s3
    //         .download_from_aws_s3((&photo, &img_size))
    //         .await
    //         .context(GetAWSObjectSnafu)?;

    //     response.body
    // };
    //

    let mut stream = cache.stream(photo, &img_size).await.context(CacheSnafu)?;

    Ok((
        Status::Ok,
        (
            ContentType::JPEG,
            ByteStream! {
                while let Some(bytes) = stream.next().await {
                    match bytes {
                        Ok(chunk) => {
                            yield chunk
                        },
                        Err(err) => {
                            let error = Error::Stream { source: err };
                            error!("Failed to read chunk: {}", error);
                            break
                        }
                    }
                }
            },
        ),
    ))
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid size '{}': {}", size, source))]
    Size { size: String, source: ParseError },

    #[snafu(display("Failed to get connection: {}", source))]
    Connection { source: SqlxError },

    #[snafu(display("Failed to get photo by id: {}", source))]
    Photo { source: PhotoDbError },

    #[snafu(display("Failed to download photo: {}", source))]
    Stream { source: ByteStreamError },

    #[snafu(display("Failed to get image from cache: {}", source))]
    Cache { source: CacheError },
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status: Status = match &self {
            Error::Size { .. } => Status::InternalServerError,
            Error::Connection { .. } => Status::InternalServerError,
            Error::Photo { .. } => Status::InternalServerError,
            Error::Stream { .. } => Status::InternalServerError,
            Error::Cache { source } => match source {
                CacheError::GetAWSObject { .. } => Status::NotFound,
                CacheError::Stream { .. } => Status::InternalServerError,
            },
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
            Error::Cache { source } => {
                state.serialize_field("kind", "Invalid Photo")?;
                state.serialize_field("message", "")?;

                error!("Failed to get AWS Object: {}", source);
            }
        };

        state.end()
    }
}
