use crate::state::AppState;
use crate::TEMPLATES;
use actix_web::{get, web, HttpResponse, Responder};
use rand::seq::SliceRandom;
use tera::Context;

#[get("/")]
pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut context = Context::new();

    data.portfolio_photos
        .choose_multiple(&mut rand::thread_rng(), 3)
        .enumerate()
        .for_each(|(i, p)| {
            println!("Inserted photo {} into context: {}", i, p.id);

            context.insert(format!("photo_{}", i), p);
        });

    context.insert("api_host", &data.api_host);

    let content = TEMPLATES.render("index.html", &context).unwrap();

    HttpResponse::Ok().body(content)
}
