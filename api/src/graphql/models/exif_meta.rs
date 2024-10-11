use crate::graphql::{
    loaders::{fujifilm_recipe::FujifilmRecipeByExifMetaId, AppLoader},
    models::FujifilmRecipe,
};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, Enum, Result, SimpleObject, ID,
};
use core_victorhqc_com::models::exif_meta::{ExifMeta as ExifMetaModel, Maker as CoreMaker};

#[derive(Clone, Copy, Debug, Enum, Eq, PartialEq)]
pub enum Maker {
    Fujifilm,
    Konica,
    Canon,
}

impl From<CoreMaker> for Maker {
    fn from(value: CoreMaker) -> Self {
        match value {
            CoreMaker::Fujifilm => Maker::Fujifilm,
            CoreMaker::Konica => Maker::Konica,
            CoreMaker::Canon => Maker::Canon,
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
    pub focal_length: FocalLength,
    pub exposure_compensation: f64,
    pub aperture: f64,
    pub maker: Maker,
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
            date_taken: value.details.date_taken.map(|d| d.0),
            iso: value.details.iso.0,
            focal_length: FocalLength {
                value: value.details.focal_length.value,
                eq_35mm: value.details.focal_length.eq_35mm,
                crop_factor: value.details.focal_length.crop_factor,
            },
            exposure_compensation: value.details.exposure_compensation.0,
            aperture: value.details.aperture.0,
            maker: value.details.maker.into(),
            camera_name: value.details.camera_name,
            lens_name: value.details.lens_name,
        }
    }
}
