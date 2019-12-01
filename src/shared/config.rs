use std::env;

#[derive(Debug)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub id: String,
    pub secret: String,
}

impl Config {
    pub fn load() -> Config {
        Config {
            username: env::var("REDDIT_USERNAME").unwrap(),
            password: env::var("REDDIT_PASSWORD").unwrap(),
            id: env::var("REDDIT_APP_ID").unwrap(),
            secret: env::var("REDDIT_APP_SECRET").unwrap(),
        }
    }
}
