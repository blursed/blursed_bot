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

mod chat;
mod shared;

use crate::chat::handler::Handler;
use crate::shared::config::Config;

fn main() {

    let config = Config::load();
    let mut chat_handler = Handler;
    chat_handler.listen_and_respond(&config.slack_bot_oauth_access_token);
}
