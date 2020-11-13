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
use chat::slack::{transform_reddit_search_hit_to_payload, IncomingMessage, OutgoingMessage};
use net::auth;
use std::env;

#[post("/")]
async fn index(
    form: web::Form<IncomingMessage>,
    request: web::Data<reqwest::Client>,
    config: web::Data<Config>,
) -> impl Responder {
    let reddit_client = RedditClient::new(&config, &request);
    let params = [("q", &form.text), ("restrict_sr", &"true".to_owned())];
    // TODO: Handle unsafe .unwrap here
    let random_search_hit = reddit_client.blursed_search(&params).unwrap();
    let message = transform_reddit_search_hit_to_payload(&random_search_hit);

    web::Json(message)
}

#[get("/ping")]
async fn ping() -> impl Responder {
    "pong".to_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let request = reqwest::Client::new();
    let config = Config::load();
    let address = format!(
        "{}:{:?}",
        &config.http_host.to_owned(),
        &config.http_port.to_owned()
    );

    HttpServer::new(move || {
        App::new()
            .data(request.clone())
            .data(config.clone())
            .service(index)
            .service(ping)
    })
    .bind(address)?
    .run()
    .await
}
