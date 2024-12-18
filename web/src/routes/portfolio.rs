use super::context::render_content;
use crate::{requests, state::AppState};
use actix_web::{error::ResponseError, get, web, HttpResponse, Responder, Result};
use snafu::prelude::*;
use tera::Context;

#[get("/portfolio")]
pub async fn portfolio(data: web::Data<AppState>) -> Result<impl Responder> {
    let mut context = Context::new();
    let portfolio = requests::photos::get_photos_from_tag("portfolio")
        .await
        .context(PortfolioSnafu)?;

    context.insert("portfolio_photos", &portfolio);

    let content = render_content("portfolio", &mut context, &data)?;

    Ok(HttpResponse::Ok().body(content))
}

#[derive(Debug, Snafu)]
pub enum Error {
    Portfolio { source: requests::photos::Error },
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Portfolio { source } => {
                HttpResponse::InternalServerError().body(source.to_string())
            }
        }
    }
}
