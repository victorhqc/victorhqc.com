use crate::graphql::queries::RootQuery;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub type RootSchema = Schema<RootQuery, EmptyMutation, EmptySubscription>;

// #[Object]
// impl QueryRoot {
//     async fn photo
// }
