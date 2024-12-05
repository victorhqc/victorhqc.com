use actix_files as fs;
use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
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

#[get("/")]
async fn hello() -> impl Responder {
    let context = Context::new();
    let content = TEMPLATES.render("index.html", &context).unwrap();

    HttpResponse::Ok().body(content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(fs::Files::new("/static", "./static"))
            .service(hello)
    })
    .workers(4)
    .bind(("127.0.0.1", 7879))?
    .run()
    .await
}
