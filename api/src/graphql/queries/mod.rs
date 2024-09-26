use async_graphql::MergedObject;

mod photo;
mod exif_meta;

#[derive(MergedObject, Default)]
pub struct RootQuery(photo::PhotoQuery, exif_meta::ExifMetaQuery);
