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
use serde_json::json;
use std::env;

#[post("/")]
async fn index(
    form: web::Form<IncomingMessage>,
    request: web::Data<reqwest::Client>,
    config: web::Data<Config>,
) -> impl Responder {
    let reddit_client = RedditClient::new(&config, &request);
    let params = [("q", "waiting"), ("restrict_sr", "true")];
    // TODO: Handle unsafe .unwrap here
    let random_search_hit = reddit_client.blursed_search(&params).unwrap();

    let payload = json!({
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "This is a section block with an accessory image."
                },
                "accessory": {
                    "type": "image",
                    "image_url": &random_search_hit.url.to_owned(),
                    "alt_text": "cute cat"
                }
            }
        ]
    });

    let message = OutgoingMessage {
        response_type: "in_channel".to_string(),
        text: payload.to_string(),
    };

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
