pub mod exif_meta;
pub mod fujifilm;
pub mod photo;
pub mod tag;

#[derive(sqlx::Type, Debug, Clone)]
#[sqlx(transparent)]
pub struct Timestamp(i64);
