//! blursed_bot - blessed and blursed & cursed images
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]

use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct Message {
    response_type: String,
    text: String,
}

#[post("/")]
async fn index(_info: web::Path<()>) -> impl Responder {
    let message = Message{
        response_type: "in_channel".to_string(),
        text: "Hello from blursed_bot!".to_string(),
    };
    web::Json(message)
}

#[get("/ping")]
async fn ping(_info: web::Path<()>) -> impl Responder {
    format!("pong")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port_string = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let port = port_string.parse().expect("PORT must be a number");
    HttpServer::new(|| App::new()
        .service(index)
        .service(ping)
    )
    .bind(("0.0.0.0", port)).expect("Cannot bind to port").run().await
}
