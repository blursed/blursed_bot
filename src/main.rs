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

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/index.json")]
async fn index(_info: web::Path<()>) -> impl Responder {
    format!("Hi it's blursed bot")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
