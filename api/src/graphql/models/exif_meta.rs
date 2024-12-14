use crate::graphql::{
    loaders::{fujifilm_recipe::FujifilmRecipeByExifMetaId, AppLoader},
    models::FujifilmRecipe,
};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, Enum, Result, SimpleObject, ID,
};
use core_victorhqc_com::models::exif_meta::{
    CameraMaker as CoreCameraMaker, ExifMeta as ExifMetaModel, LensMaker as CoreLensMaker,
};

#[derive(Clone, Copy, Debug, Enum, Eq, PartialEq)]
pub enum CameraMaker {
    Fujifilm,
    Konica,
    Canon,
}

#[derive(Clone, Copy, Debug, Enum, Eq, PartialEq)]
pub enum LensMaker {
    Fujifilm,
    Konica,
    Canon,
    SevenArtisans,
    Unknown,
}

impl From<CoreCameraMaker> for CameraMaker {
    fn from(value: CoreCameraMaker) -> Self {
        match value {
            CoreCameraMaker::Fujifilm => Self::Fujifilm,
            CoreCameraMaker::Konica => Self::Konica,
            CoreCameraMaker::Canon => Self::Canon,
        }
    }
}

impl From<CoreLensMaker> for LensMaker {
    fn from(value: CoreLensMaker) -> Self {
        match value {
            CoreLensMaker::Fujifilm => Self::Fujifilm,
            CoreLensMaker::Konica => Self::Konica,
            CoreLensMaker::Canon => Self::Canon,
            CoreLensMaker::SevenArtisans => Self::SevenArtisans,
            CoreLensMaker::Unknown => Self::Unknown,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct FocalLength {
    pub value: f64,
    pub eq_35mm: f64,
    pub crop_factor: f64,
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct ExifMeta {
    pub id: ID,
    pub rating: i64,
    pub city: Option<String>,
    pub date_taken: Option<String>,
    pub iso: i64,
    pub aperture: f64,
    pub shutter_speed: String,
    pub focal_length: FocalLength,
    pub exposure_compensation: f64,
    pub camera_maker: CameraMaker,
    pub lens_maker: LensMaker,
    pub camera_name: String,
    pub lens_name: Option<String>,
}

#[ComplexObject]
impl ExifMeta {
    async fn fujifilm_recipe(&self, ctx: &Context<'_>) -> Result<Option<FujifilmRecipe>> {
        let loader = ctx.data_unchecked::<DataLoader<AppLoader>>();
        let id = FujifilmRecipeByExifMetaId::new(&self.id);

        let recipe = loader.load_one(id).await?;

        Ok(recipe)
    }
}

impl From<ExifMetaModel> for ExifMeta {
    fn from(value: ExifMetaModel) -> Self {
        ExifMeta {
            id: value.id.into(),
            rating: value.details.rating.0.into(),
            city: value.details.city.map(|c| c.0),
            date_taken: value.details.date_taken.map(|d| format!("{}", d.0)),
            iso: value.details.iso.0,
            aperture: value.details.aperture.0,
            shutter_speed: value.details.shutter_speed.0,
            focal_length: FocalLength {
                value: value.details.focal_length.value,
                eq_35mm: value.details.focal_length.eq_35mm,
                crop_factor: value.details.focal_length.crop_factor,
            },
            exposure_compensation: value.details.exposure_compensation.0,
            camera_maker: value.details.camera_maker.into(),
            lens_maker: value.details.lens_maker.into(),
            camera_name: value.details.camera_name,
            lens_name: value.details.lens_name,
        }
    }
}
