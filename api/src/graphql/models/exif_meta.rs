use crate::{
    graphql::{
        loaders::{fujifilm_recipe::FujifilmRecipeByExifMetaId, AppLoader},
        models::FujifilmRecipe,
    },
    models::exif_meta::{ExifMeta as ExifMetaModel, Maker},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Result, SimpleObject, ID};

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
            maker: value.maker,
            crop_factor: value.crop_factor,
            camera_name: value.camera_name,
            lens_name: value.lens_name,
        }
    }
}
