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

use crate::chat::handler::Handler;

fn main() {
    
    // TODO Put this env var in Config object
    match std::env::var("SLACK_BOT_OAUTH_ACCESS_TOKEN") {
        Ok(token) => {
            let mut chat_handler = Handler;
            let r = slack::RtmClient::login_and_run(&token, &mut chat_handler);
            match r {
                Ok(_) => {}
                Err(err) => panic!("Error: {}", err),
            }
        },
        _ => {
            panic!("Error: SLACK_BOT_OAUTH_ACCESS_TOKEN environment variable not found")
        },
    }
}
