extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub slack_bot_oauth_access_token: String,
}

impl Config {

    pub fn load() -> Config {
        dotenv().ok();

        Config {
            slack_bot_oauth_access_token: match env::var("SLACK_BOT_OAUTH_ACCESS_TOKEN") {
                Ok(s) => s,
                Err(_err) => panic!("Env Error: SLACK_BOT_OAUTH_ACCESS_TOKEN does not exist"),
            },
        }
    }
}
