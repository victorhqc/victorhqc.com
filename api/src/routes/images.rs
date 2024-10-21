use crate::AppState;
use core_victorhqc_com::{
    aws::{
        image_size::{Error as ParseError, ImageSize},
        photo::{ByteStreamError, Error as AWSError},
    },
    models::photo::{db::Error as PhotoDbError, Photo},
    sqlx::Error as SqlxError,
};
use log::error;
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

#[get("/image/<id>/<size>")]
pub async fn get_image(
    id: String,
    size: String,
    state: &State<AppState>,
) -> Result<(Status, (ContentType, ByteStream![Bytes])), Error> {
    let s3 = &state.s3;
    let pool = &state.db_pool;
    let mut conn = pool.acquire().await.context(ConnectionSnafu)?;

    let img_size: ImageSize =
        ImageSize::from_str(&size).context(SizeSnafu { size: size.clone() })?;

    let photo = Photo::find_by_id(&mut conn, &id)
        .await
        .context(PhotoSnafu)?;

    let response = s3
        .download_from_aws_s3((&photo, img_size))
        .await
        .context(GetAWSObjectSnafu)?;

    Ok((
        Status::Ok,
        (
            ContentType::JPEG,
            ByteStream! {
                let mut body = response.body;

                while let Some(bytes) = body.next().await {
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
