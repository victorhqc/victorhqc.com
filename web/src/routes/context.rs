use crate::{state::AppState, TEMPLATES};
use actix_web::{web, ResponseError, Result};
use snafu::prelude::*;
use tera::Context;

pub fn render_content(
    route: &str,
    ctx: &mut Context,
    data: &web::Data<AppState>,
) -> Result<String> {
    let is_production = false;
    #[cfg(not(debug_assertions))]
    let is_production = true;

    ctx.insert("api_host", &data.api_host);
    ctx.insert("is_production", &is_production);

    let content = TEMPLATES
        .render(format!("{}.html", route).as_str(), ctx)
        .context(TemplateSnafu {
            route: route.to_string(),
        })?;

    Ok(content)
}

#[derive(Debug, Snafu)]
pub enum Error {
    Template { source: tera::Error, route: String },
}

impl ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            Error::Template { source, route } => actix_web::HttpResponse::InternalServerError()
                .body(format!("{}: {}", route, source)),
        }
    }
}
