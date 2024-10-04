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
#[graphql(complex)]
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
            iso: value.iso,
            focal_length: value.focal_length,
            exposure_compensation: value.exposure_compensation,
            aperture: value.aperture,
            maker: value.maker.into(),
            crop_factor: value.crop_factor,
            camera_name: value.camera_name,
            lens_name: value.lens_name,
        }
    }
}
