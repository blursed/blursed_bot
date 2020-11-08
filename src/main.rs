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
#[macro_use]
extern crate serde;
extern crate dotenv;
mod chat;

mod net;
mod shared;

use crate::net::auth::Auth;
use crate::net::reddit_client::RedditClient;
use crate::shared::config::Config;
use actix_web::{get, post, web, App, HttpServer, Responder};
use chat::slack::{IncomingMessage, OutgoingMessage};
use net::auth;
use std::env;

#[post("/")]
async fn index(form: web::Form<IncomingMessage>) -> impl Responder {
    let message = OutgoingMessage {
        response_type: "in_channel".to_string(),
        text: format!("You typed: {}", form.text),
    };
    web::Json(message)
}

#[get("/ping")]
async fn ping(request: web::Data<reqwest::Client>, config: web::Data<Config>) -> impl Responder {
    let reddit_client = RedditClient::new(&config, &request);
    let params = [("q", "waiting"), ("restrict_sr", "true")];
    let random_search_hit = reddit_client.blursed_search(&params);
    println!("{:?} random search result!", random_search_hit);
    "pong".to_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let request = reqwest::Client::new();
    let config = Config::load();
    let port_string = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let port = port_string.parse().expect("PORT must be a number");

    HttpServer::new(move || {
        App::new()
            .data(request.clone())
            .data(config.clone())
            .service(index)
            .service(ping)
    })
    .bind(("0.0.0.0", port))
    .expect("Cannot bind to port")
    .run()
    .await
}
