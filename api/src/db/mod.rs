pub mod photos;

#[derive(sqlx::Type, Debug, Clone, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct Timestamp(i64);
