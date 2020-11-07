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

use crate::chat::handler::Handler;

mod net;
mod shared;

use crate::net::auth::Auth;
use crate::net::reddit_client::RedditClient;
use crate::shared::config::Config;
use net::auth;

fn main() {
    /*let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => panic!("No api-key in args! Usage: cargo run --example slack_example -- <api-key>"),
        x => args[x - 1].clone(),
    };
    let mut chat_handler = Handler;
    let r = slack::RtmClient::login_and_run(&api_key, &mut chat_handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    } */
    let client = reqwest::Client::new();
    let config = Config::load();
    let reddit_client = RedditClient::new(&config, &client);
    let params = [("q", "waiting"), ("restrict_sr", "true")];
    let search_result = reddit_client.blursed_search(&params);
    //::<SearchResponse>
    println!("{:?} search result!", search_result);
    println!("api url {:?}", config.api_url("test"));
}
