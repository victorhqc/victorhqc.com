use crate::graphql::{mutations::RootMutation, queries::RootQuery};
use async_graphql::{EmptySubscription, Schema};

pub type RootSchema = Schema<RootQuery, RootMutation, EmptySubscription>;
