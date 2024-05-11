pub mod fujifilm;

use time::OffsetDateTime;

pub struct Photo {
    pub id: String,
    pub src: String,
    pub filename: String,
    pub filetype: FileType,
    pub date_taken: OffsetDateTime,
    pub city: String,
    pub exif_meta_id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted: bool,
}

pub struct ExifMeta {
    pub id: String,
    pub iso: i32,
    pub focal_length: f32,
    pub aperture: f32,
    pub maker: Maker,
    pub crop_factor: f32,
    pub camera_name: String,
    pub lens_name: Option<String>,
    fuji_recipe_id: Option<String>
}

pub enum FileType {
    JPEG,
}

pub enum Maker {
    FUJIFILM,
    KONICA,
    CANON
}
