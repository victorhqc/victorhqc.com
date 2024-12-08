use crate::state::AppState;
use crate::TEMPLATES;
use actix_web::{get, web, HttpResponse, Responder};
use rand::seq::SliceRandom;
use tera::Context;

#[get("/")]
pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let random_photo_id = data
        .random_photo_ids
        .choose(&mut rand::thread_rng())
        .unwrap();

    let mut context = Context::new();
    context.insert("id", random_photo_id);
    context.insert("api_host", &data.api_host);

    let content = TEMPLATES.render("index.html", &context).unwrap();

    HttpResponse::Ok().body(content)
}
