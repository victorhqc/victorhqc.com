use std::net::Ipv4Addr;

use crate::engine::{AppEngine, CustomKey};
use axum::{extract::FromRef, response::IntoResponse, routing::get, serve, Router};
use axum_template::{engine::Engine, RenderHtml};
use serde::Serialize;
use tera::Tera;
use tokio::net::TcpListener;

mod engine;

#[derive(Debug, Serialize)]
struct Data {}

async fn index(
    // Obtain the engine
    engine: AppEngine,
    // Extract the custom key
    CustomKey(template): CustomKey,
    // Path(name): Path<String>,
) -> impl IntoResponse {
    RenderHtml(template, engine, Data {})
}

#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}

#[tokio::main]
async fn main() {
    let tera = Tera::new("templates/**/*.html").expect("Template folder not found");
    let app = Router::new().route("/", get(index)).with_state(AppState {
        engine: Engine::from(tera),
    });

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 7879))
        .await
        .unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
