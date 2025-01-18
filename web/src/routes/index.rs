use super::context::{render_content, RenderArgs, TemplateKind};
use super::get_user_agent;
use crate::{
    analytics::routes, collections::Collection, gql::get_portfolio::GetPortfolioPhotos,
    state::AppState,
};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};
use log::debug;
use rand::seq::SliceRandom;
use tera::Context;

#[get("/")]
pub async fn index(data: web::Data<AppState>, req: HttpRequest) -> Result<impl Responder> {
    let ua = get_user_agent(&req);
    let mut context = Context::new();

    let prefetched = &data.prefetched;
    let portfolio_photos = prefetched.get(&Collection::Portfolio).unwrap();

    let random_photos: Vec<&GetPortfolioPhotos> = portfolio_photos
        .choose_multiple(&mut rand::thread_rng(), 3)
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
