use super::context::{render_content, TemplateKind};
use crate::{gql::get_portfolio, state::AppState, Collection};
use actix_quick_extract::headers::UserAgent;
use actix_web::{get, web, HttpResponse, Responder, Result};
use rand::seq::SliceRandom;
use tera::Context;
use uaparser::Parser;

#[get("/")]
pub async fn index(data: web::Data<AppState>, user_agent: UserAgent) -> Result<impl Responder> {
    let parsed = data.ua_parser.parse(&user_agent.0);
    println!("PARSED {:?}", parsed);

    let mut context = Context::new();
    let prefetched = &data.prefetched;
    let portfolio_photos = prefetched.get(&Collection::Portfolio).unwrap();

    let random_photos: Vec<&get_portfolio::GetPortfolioPhotos> = portfolio_photos
        .choose_multiple(&mut rand::thread_rng(), 3)
        .collect();

    context.insert("photos", &random_photos);

    let content = render_content("index", TemplateKind::Html, &mut context, &data)?;

    Ok(HttpResponse::Ok().body(content))
}
