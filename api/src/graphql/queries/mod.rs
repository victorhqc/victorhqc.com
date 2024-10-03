use async_graphql::MergedObject;

mod exif_meta;
mod fujifilm_recipe;
mod photo;
mod tag;

#[derive(MergedObject, Default)]
pub struct RootQuery(
    photo::PhotoQuery,
    exif_meta::ExifMetaQuery,
    tag::TagQuery,
    fujifilm_recipe::FujifilmRecipeQuery,
);
