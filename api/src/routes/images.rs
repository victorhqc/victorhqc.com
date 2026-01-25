use crate::AppState;
use crate::cache::image_cache::Error as CacheError;
use core_victorhqc_com::{
    aws::image_size::{Error as ParseError, ImageSize, ImageType},
    models::photo::{Photo, db::Error as PhotoDbError},
    sqlx::Error as SqlxError,
};
use log::{debug, error};
use rocket::{
    Request, Response, State,
    http::{ContentType, Header, Status},
    request::{FromRequest, Outcome},
    response::Responder,
    serde::json::serde_json,
};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use snafu::prelude::*;
use std::io::Cursor;
use std::str::FromStr;

#[get("/images/<size>/<id>?<kind>")]
pub async fn get_image(
    size: &str,
    id: &str,
    kind: Option<&str>,
    state: &State<AppState>,
    if_none_match: IfNoneMatch,
) -> Result<ImageResponse, Error> {
    let pool = &state.db_pool;
    let cache = &state.img_cache;
    let mut conn = pool.acquire().await.context(ConnectionSnafu)?;

    let img_size: ImageSize = ImageSize::from_str(size).context(SizeSnafu {
        size: size.to_string(),
    })?;

    let kind = kind.map_or("webp", |k| k);
    let img_kind: ImageType = ImageType::from_str(kind).context(KindSnafu {
        kind: kind.to_string(),
    })?;

    debug!("id: {}", id);

    if let Some(etag) = if_none_match.0
        && cache.md5_exists(&etag).await
    {
        debug!("Cache Hit");

        return Ok(ImageResponse::not_modified(etag, img_kind));
    };

    debug!("Cache Miss");

    let photo = Photo::find_by_id(&mut conn, id).await.context(PhotoSnafu)?;

    let (etag, data) = cache
        .get(photo, &img_kind, &img_size)
        .await
        .context(CacheSnafu)?;

    Ok(ImageResponse::ok(data, etag, img_kind))
}

pub struct ImageResponse {
    data: Option<Vec<u8>>,
    kind: ImageType,
    etag: String,
    not_modified: bool,
}

impl ImageResponse {
    pub fn ok(data: Vec<u8>, etag: String, kind: ImageType) -> Self {
        Self {
            data: Some(data),
            kind,
            etag,
            not_modified: false,
        }
    }

    pub fn not_modified(etag: String, kind: ImageType) -> Self {
        Self {
            data: None,
            kind,
            etag,
            not_modified: true,
        }
    }
}

impl<'r> Responder<'r, 'static> for ImageResponse {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'static>, Status> {
        let content_type = match self.kind {
            ImageType::Jpeg => "image/jpeg",
            ImageType::Webp => "image/webp",
        };

        let mut response = Response::build();
        response
            .header(Header::new("Content-Type", content_type))
            .header(Header::new("ETag", self.etag))
            .header(Header::new(
                "Cache-Control",
                "public, max-age=31536000, immutable",
            ));

        if self.not_modified {
            // 304 Not Modified - no body
            response.status(Status::NotModified);
        } else if let Some(data) = self.data {
            // 200 OK - with body
            response.sized_body(data.len(), Cursor::new(data));
        } else {
            return Err(Status::InternalServerError);
        }

        response.ok()
    }
}

pub struct IfNoneMatch(Option<String>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IfNoneMatch {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let header_value = req.headers().get_one("If-None-Match");
        Outcome::Success(IfNoneMatch(header_value.map(String::from)))
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid size '{}': {}", size, source))]
    Size { size: String, source: ParseError },

    #[snafu(display("Invalid type '{}': {}", kind, source))]
    Kind { kind: String, source: ParseError },

    #[snafu(display("Failed to get connection: {}", source))]
    Connection { source: SqlxError },

    #[snafu(display("Failed to get photo by id: {}", source))]
    Photo { source: PhotoDbError },

    #[snafu(display("Failed to get image from cache: {}", source))]
    Cache { source: CacheError },
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status: Status = match &self {
            Error::Size { .. } => Status::InternalServerError,
            Error::Kind { .. } => Status::BadRequest,
            Error::Connection { .. } => Status::InternalServerError,
            Error::Photo { .. } => Status::InternalServerError,
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
            Error::Kind { kind, .. } => {
                state.serialize_field("kind", &format!("Invalid type: {}", kind))?;
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
            Error::Cache { source } => {
                state.serialize_field("kind", "Invalid Photo")?;
                state.serialize_field("message", "")?;

                error!("Failed to get AWS Object: {}", source);
            }
        };

        state.end()
    }
}
