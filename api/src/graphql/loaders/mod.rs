use sqlx::{Pool, Sqlite};

pub mod photo;

pub struct AppLoader {
    pub pool: Pool<Sqlite>,
}

impl AppLoader {
    pub fn default(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}
