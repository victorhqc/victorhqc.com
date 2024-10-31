use async_graphql::MergedObject;

mod photo;

#[derive(MergedObject, Default)]
pub struct RootMutation(photo::PhotoMutation);
