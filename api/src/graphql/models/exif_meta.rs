use crate::models::Maker;
use crate::models::exifmeta::ExifMeta as ExifMetaModel;
use async_graphql::{SimpleObject, ID};

#[derive(SimpleObject, Clone)]
pub struct ExifMeta {
    pub id: ID,
    pub iso: i64,
    pub focal_length: f64,
    pub exposure_compensation: f64,
    pub aperture: f64,
    pub maker: Maker,
    pub crop_factor: f64,
    pub camera_name: String,
    pub lens_name: Option<String>,
}

impl From<ExifMetaModel> for ExifMeta {
    fn from(value: ExifMetaModel) -> Self {
        ExifMeta {
            id: value.id.into(),
            iso: value.iso,
            focal_length: value.focal_length,
            exposure_compensation: value.exposure_compensation,
            aperture: value.aperture,
            maker: value.maker,
            crop_factor: value.crop_factor,
            camera_name: value.camera_name,
            lens_name: value.lens_name,
        }
    }
}
