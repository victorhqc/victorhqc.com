use crate::graphql::{context::get_conn, models::Photo as GqlPhoto};
use async_graphql::{Context, InputObject, Object, Result};

#[derive(Default)]
pub struct PhotoMutation;

#[derive(InputObject)]
pub struct CreateInput {
    title: String,
}

#[Object]
impl PhotoMutation {
    async fn create_photo(&self, _ctx: &Context<'_>, _input: CreateInput) -> Result<GqlPhoto> {
        todo!();
    }
}
