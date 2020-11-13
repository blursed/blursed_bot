use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub id: String,
    pub secret: String,
    pub access_token_url: String,
    pub api_base_url: String,
    pub http_host: String,
    pub http_port: u16,
}

impl Config {
    pub fn load() -> Config {
        dotenv().ok();
        Config {
            http_host: match env::var("HTTP_HOST") {
                Ok(s) => s,
                _ => "0.0.0.0".to_owned(),
            },
            http_port: match env::var("HTTP_PORT") {
                Ok(s) => s.parse().unwrap_or(3000),
                _ => 3000,
            },
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
            access_token_url: match env::var("REDDIT_API_ACCESS_TOKEN_URL") {
                Ok(s) => s,
                Err(_e) => panic!("Env Error: REDDIT_API_ACCESS_TOKEN_URL does not exist"),
            },
            api_base_url: match env::var("REDDIT_API_BASE_URL") {
                Ok(s) => s,
                Err(_e) => panic!("Env Error: REDDIT_API_BASE_URL does not exist"),
            },
        }
    }
    pub fn api_url(&self, path: &str) -> String {
        format!("{}/{}", &self.api_base_url, path)
    }
}
