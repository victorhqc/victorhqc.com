use sqlx::{Pool, Sqlite};

pub mod exif_meta;
pub mod fujifilm_recipe;
pub mod photo;
pub mod tag;

pub struct AppLoader {
    pub pool: Pool<Sqlite>,
}

impl AppLoader {
    pub fn default(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}
