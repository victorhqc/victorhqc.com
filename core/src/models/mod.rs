use time::OffsetDateTime;

pub mod exif_meta;
pub mod fujifilm;
pub mod photo;
pub mod tag;

#[derive(sqlx::Type, Debug, Clone)]
#[sqlx(transparent)]
pub struct Timestamp(pub i64);

impl From<OffsetDateTime> for Timestamp {
    fn from(dt: OffsetDateTime) -> Self {
        Timestamp(dt.unix_timestamp())
    }
}
