use async_graphql::{MergedObject, Schema};

mod photo;

#[derive(MergedObject, Default)]
pub struct RootQuery(photo::PhotoQuery);

