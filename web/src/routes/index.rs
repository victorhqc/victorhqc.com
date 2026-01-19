use super::context::{RenderArgs, TemplateKind, render_content};
use super::get_user_agent;
use crate::gql::get_portfolio::Orientation;
use crate::{
    analytics::routes, collections::Collection, gql::get_portfolio::GetPortfolioPhotos,
    state::AppState,
};
use actix_web::{HttpRequest, HttpResponse, Responder, Result, get, web};
use log::debug;
use rand::seq::SliceRandom;
use tera::Context;

#[get("/")]
pub async fn index(data: web::Data<AppState>, req: HttpRequest) -> Result<impl Responder> {
    let ua = get_user_agent(&req);
    let mut context = Context::new();

    let prefetched = &data.prefetched;
    let portfolio_photos = prefetched.get(&Collection::Portfolio).unwrap();

    // Only landscape photos for the main stack
    let filtered_photos: Vec<&GetPortfolioPhotos> = portfolio_photos
        .iter()
        .filter(|photo| photo.orientation == Orientation::LANDSCAPE)
        .collect();

    let random_photos: Vec<&GetPortfolioPhotos> = filtered_photos
        .choose_multiple(&mut rand::thread_rng(), 3)
        .copied()
        .collect();

    context.insert("photos", &random_photos);

    let args = RenderArgs {
        route: "index",
        route_to_record: Some(routes::Route::Index),
        kind: TemplateKind::Html,
        ctx: &mut context,
        data: &data,
        user_agent: ua.get(),
    };

    let content = render_content(args)?;

    debug!("----");
    debug!(" ");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(content))
}
