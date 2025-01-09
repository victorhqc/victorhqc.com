use graphql_client::GraphQLQuery;

// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../api/schema.gql",
    query_path = "src/gql/queries/get-portfolio.graphql",
    response_derives = "Debug, Clone, Serialize"
)]
pub struct GetPortfolio;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../api/schema.gql",
    query_path = "src/gql/queries/get-one.graphql",
    response_derives = "Debug, Clone, Serialize"
)]
pub struct GetOne;
