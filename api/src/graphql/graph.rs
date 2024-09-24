use async_graphql::{Object, Schema, EmptyMutation, EmptySubscription};
use crate::graphql::queries::RootQuery;

pub type RootSchema = Schema<RootQuery, EmptyMutation, EmptySubscription>;

// #[Object]
// impl QueryRoot {
//     async fn photo
// }
