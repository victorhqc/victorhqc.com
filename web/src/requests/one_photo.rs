use crate::gql::{get_one, GetOne};
use graphql_client::{Error as GraphQLError, GraphQLQuery, Response};
use reqwest::Error as ReqwestError;
use snafu::prelude::*;

type Photo = get_one::GetOnePhoto;

pub async fn get_one_photo(id: String) -> Result<Photo, Error> {
    let api_host = std::env::var("WEB_API_HOST").expect("WEB_API_HOST env variable is missing");

    let variables = get_one::Variables { id };

    let request_body = GetOne::build_query(variables);

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/graphql", api_host))
        .json(&request_body)
        .send()
        .await
        .context(RequestSnafu)?;

    let response_body: Response<get_one::ResponseData> =
        response.json().await.context(JsonParseSnafu)?;

    if let Some(errors) = response_body.errors {
        return Err(Error::GQLErrors { errors });
    }

    if let Some(data) = response_body.data {
        return Ok(data.photo);
    }

    Err(Error::NoData)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Request failed: {:?}", source))]
    Request { source: ReqwestError },

    #[snafu(display("Request Json Deserialization failed: {:?}", source))]
    JsonParse { source: ReqwestError },

    #[snafu(display("No data from Request"))]
    NoData,

    #[snafu(display("Failed to get one photo: {:?}", errors))]
    GQLErrors { errors: Vec<GraphQLError> },
}
