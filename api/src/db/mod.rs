pub mod photos;

#[derive(sqlx::Type, Debug, Clone)]
#[sqlx(transparent)]
pub struct Timestamp(i64);
