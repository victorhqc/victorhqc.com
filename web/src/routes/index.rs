use crate::gql::get_portfolio;
use crate::state::AppState;
use crate::TEMPLATES;
use actix_web::{get, web, HttpResponse, Responder};
use rand::seq::SliceRandom;
use tera::Context;

#[get("/")]
pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut context = Context::new();

    let is_production = false;
    #[cfg(not(debug_assertions))]
    let is_production = true;

    let random_photos: Vec<&get_portfolio::GetPortfolioPhotos> = data
        .portfolio_photos
        .choose_multiple(&mut rand::thread_rng(), 3)
        .map(|p| p)
        .collect();

    context.insert("photos", &random_photos);
    context.insert("api_host", &data.api_host);
    context.insert("is_production", &is_production);

    let content = TEMPLATES.render("index.html", &context).unwrap();

    HttpResponse::Ok().body(content)
}
