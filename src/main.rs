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
use net::auth;
use actix_web::{get, post, web, App, HttpServer, Responder};
use chat::slack::{IncomingMessage, OutgoingMessage};
use std::env;

#[post("/")]
async fn index(form: web::Form<IncomingMessage>) -> impl Responder {
    let message = OutgoingMessage{
        response_type: "in_channel".to_string(),
        text: format!("You typed: {}", form.text),
    };
    web::Json(message)
}

#[get("/ping")]
async fn ping(_info: web::Path<()>) -> impl Responder {
    let client = reqwest::Client::new();
    let config = Config::load();
    let reddit_client = RedditClient::new(&config, &client);
    let params = [("q", "waiting"), ("restrict_sr", "true")];
    let search_result = reddit_client.blursed_search(&params);
    println!("{:?} search result!", search_result);
    println!("api url {:?}", config.api_url("test"));
    "pong".to_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port_string = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let port = port_string.parse().expect("PORT must be a number");
    HttpServer::new(|| App::new()
        .service(index)
        .service(ping)
    ).bind(("0.0.0.0", port)).expect("Cannot bind to port").run().await
}
