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
            iso: value.iso.0,
            focal_length: FocalLength {
                value: value.focal_length.value,
                eq_35mm: value.focal_length.eq_35mm,
                crop_factor: value.focal_length.crop_factor,
            },
            exposure_compensation: value.exposure_compensation.0,
            aperture: value.aperture.0,
            maker: value.maker.into(),
            camera_name: value.camera_name,
            lens_name: value.lens_name,
        }
    }
}
