use crate::gql::{get_portfolio, GetPortfolio};
use graphql_client::{Error as GraphQLError, GraphQLQuery, Response};
use reqwest::Error as ReqwestError;
use snafu::prelude::*;

type PortfolioPhotos = get_portfolio::GetPortfolioPhotos;

pub async fn get_photos_from_tag(name: &str) -> Result<Vec<PortfolioPhotos>, Error> {
    let api_host = std::env::var("WEB_API_HOST").expect("WEB_API_HOST env variable is missing");

    let variables = get_portfolio::Variables {
        tag: name.to_string(),
        max: Some(12),
    };

    let request_body = GetPortfolio::build_query(variables);

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/graphql", api_host))
        .json(&request_body)
        .send()
        .await
        .context(RequestSnafu)?;

    let response_body: Response<get_portfolio::ResponseData> =
        response.json().await.context(JsonParseSnafu)?;

    if let Some(errors) = response_body.errors {
        return Err(Error::GQLErrors { errors });
    }

    if let Some(data) = response_body.data {
        return Ok(data.photos);
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

    #[snafu(display("Failed to get portgolio photos: {:?}", errors))]
    GQLErrors { errors: Vec<GraphQLError> },
}
