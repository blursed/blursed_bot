use dotenv::dotenv;
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
        dotenv().ok();
        Config {
            username: match env::var("REDDIT_USERNAME") {
                Ok(s) => s,
                Err(_e) => panic!("Env Error: REDDIT_USERNAME does not exist"),
            },
            password: match env::var("REDDIT_PASSWORD") {
                Ok(s) => s,
                Err(_e) => panic!("Env Error: REDDIT_PASSWORD does not exist"),
            },
            id: match env::var("REDDIT_APP_ID") {
                Ok(s) => s,
                Err(_e) => panic!("Env Error: REDDIT_APP_ID does not exist"),
            },
            secret: match env::var("REDDIT_APP_SECRET") {
                Ok(s) => s,
                Err(_e) => panic!("Env Error: REDDIT_APP_SECRET does not exist"),
            },
        }
    }
}
