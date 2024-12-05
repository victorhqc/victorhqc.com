use actix_files as fs;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use serde::Deserialize;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".css"]);
        // tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

#[derive(Deserialize)]
struct Photo {
    id: String,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();

    let api_host = std::env::var("API_HOST").expect("API_HOST env variable is missing");

    let photo_ids: Vec<String> = reqwest::get(format!("{}/v1/photos/portfolio", api_host))
        .await
        .unwrap()
        .json::<Vec<Photo>>()
        .await
        .unwrap()
        .into_iter()
        .map(|photo| photo.id)
        .collect();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(AppState {
                api_host: api_host.clone(),
                random_photo_ids: photo_ids.clone(),
            }))
            .service(fs::Files::new("/static", "./static"))
            .service(hello)
    })
    .workers(4)
    .bind(("127.0.0.1", 7879))?
    .run()
    .await
}

#[derive(Debug)]
struct AppState {
    api_host: String,
    random_photo_ids: Vec<String>,
}
