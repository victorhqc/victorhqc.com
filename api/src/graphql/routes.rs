use super::graph::RootSchema;
#[cfg(debug_assertions)]
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
#[cfg(debug_assertions)]
use rocket::response::content;
use rocket::State;

#[cfg(debug_assertions)]
#[get("/graphql")]
pub fn graphql_playground() -> content::RawHtml<String> {
    content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<RootSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(
    schema: &State<RootSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}
