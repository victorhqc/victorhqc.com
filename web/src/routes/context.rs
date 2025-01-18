use crate::utils::device;
use crate::{analytics::routes::Route, state::AppState, TEMPLATES};
use actix_web::{web, ResponseError, Result};
use log::{debug, error};
use snafu::prelude::*;
use strum_macros::Display;
use tera::Context;
use uaparser::Parser;

#[derive(Debug, Display, serde::Serialize)]
pub enum TemplateKind {
    #[strum(serialize = "html")]
    #[serde(rename(serialize = "html"))]
    Html,
}

pub struct RenderArgs<'a> {
    pub route: &'a str,
    pub route_to_record: Option<Route>,
    pub kind: TemplateKind,
    pub ctx: &'a mut Context,
    pub data: &'a web::Data<AppState>,
    pub user_agent: &'a str,
}

pub fn render_content(args: RenderArgs) -> Result<String> {
    #[cfg(debug_assertions)]
    let is_production = false;
    #[cfg(not(debug_assertions))]
    let is_production = true;

    debug!("Serving file {}.{}", args.route, args.kind);

    let parsed = args.data.ua_parser.parse(args.user_agent);
    let is_mobile = device::is_mobile(&parsed.device, &parsed.os);

    if let Some(path) = args.route_to_record {
        debug!("Injecting analytics path: '{}'", path.to_string());

        args.ctx.insert("path", &path.to_string())
    };

    args.ctx.insert("api_host", &args.data.api_host);
    args.ctx.insert("is_production", &is_production);
    args.ctx.insert("is_mobile", &is_mobile);

    let content = TEMPLATES
        .render(format!("{}.{}", args.route, args.kind).as_str(), args.ctx)
        .context(TemplateSnafu {
            route: args.route.to_string(),
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
            Error::Template { source, route } => {
                error!("Failed to render route {}: {:?}", route, source);

                actix_web::HttpResponse::InternalServerError()
                    .body(format!("{}: {}", route, source))
            }
        }
    }
}
