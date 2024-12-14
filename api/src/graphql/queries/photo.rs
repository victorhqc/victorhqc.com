use crate::graphql::{context::get_conn, models::Photo as GqlPhoto};
use async_graphql::{Context, InputObject, Object, Result, ID};
use core_victorhqc_com::models::{photo::Photo, tag::Tag};

#[derive(Default)]
pub struct PhotoQuery;

#[derive(InputObject)]
pub struct PhotosQueryInput {
    pub tag: Option<String>,
    pub max_results: Option<i32>,
}

#[Object]
impl PhotoQuery {
    pub async fn photo(&self, ctx: &Context<'_>, id: ID) -> Result<GqlPhoto> {
        let mut conn = get_conn(ctx).await?;
        let photo = Photo::find_by_id(&mut conn, &id).await?;

        Ok(photo.into())
    }

    pub async fn photos(
        &self,
        ctx: &Context<'_>,
        input: PhotosQueryInput,
    ) -> Result<Vec<GqlPhoto>> {
        let mut conn = get_conn(ctx).await?;

        let photos = if let Some(tag) = input.tag {
            let tag = Tag::find_by_name(&mut conn, &tag).await?;
            let ids = vec![tag.id];

            Photo::find_by_tag_ids(&mut conn, &ids, input.max_results)
                .await?
                .into_iter()
                .map(|(_, photo)| photo)
                .collect()
        } else {
            Photo::find_all(&mut conn).await?
        };

        let photos = photos.into_iter().map(|p| p.into()).collect();

        Ok(photos)
    }
}
