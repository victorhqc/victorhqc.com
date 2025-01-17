use super::analytics::{generate_unique_etag, generate_unique_session_id, get_client_id};
use super::context::{render_content, RenderArgs, TemplateKind};
use crate::{gql::get_portfolio::GetPortfolioPhotos, state::AppState, Collection};
use actix_web::{get, http::header, web, HttpRequest, HttpResponse, Responder, Result};
use rand::seq::SliceRandom;
use tera::Context;

#[get("/")]
pub async fn index(data: web::Data<AppState>, req: HttpRequest) -> Result<impl Responder> {
    let (client_id, ua) = get_client_id(&req);
    let mut etag_map = data.visitor_etags.lock().unwrap();
    let mut session_map = data.unique_sessions.lock().unwrap();

    let session_id = session_map.get(&client_id).cloned().unwrap_or_else(|| {
        let new_session_id = generate_unique_session_id();
        session_map.insert(client_id.clone());
        new_session_id
    });

    let etag = etag_map
        .entry(session_id.clone())
        .or_insert_with(|| generate_unique_etag(&session_id))
        .clone();

    println!("client id: {:?}", client_id);
    println!("Existing ids {:?}", etag_map);

    if let Some(client_etag) = req.headers().get(header::IF_NONE_MATCH) {
        if client_etag.to_str().ok() == Some(&etag) {
            println!("Existing User");
            println!("----");
            println!(" ");

            return Ok(HttpResponse::NotModified().finish());
        }
    }

    println!("New User!");
    let new_etag = generate_unique_etag(&client_id);
    etag_map.insert(client_id.clone(), new_etag.clone());

    let mut context = Context::new();
    let prefetched = &data.prefetched;
    let portfolio_photos = prefetched.get(&Collection::Portfolio).unwrap();

    let random_photos: Vec<&GetPortfolioPhotos> = portfolio_photos
        .choose_multiple(&mut rand::thread_rng(), 3)
        .collect();
    context.insert("photos", &random_photos);

    let args = RenderArgs {
        route: "index",
        kind: TemplateKind::Html,
        ctx: &mut context,
        data: &data,
        user_agent: ua.get(),
    };

    let content = render_content(args)?;

    println!("----");
    println!(" ");

    Ok(HttpResponse::Ok()
        .insert_header((header::ETAG, new_etag))
        .content_type("text/html; charset=utf-8")
        .body(content))
}
