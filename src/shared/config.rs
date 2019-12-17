use dotenv::dotenv;
#[cfg(test)]
use mockito;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub id: String,
    pub secret: String,
    pub access_token_url: String,
    pub api_base_url: String,
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
            access_token_url: match env::var("REDDIT_API_ACCESS_TOKEN_URL") {
                Ok(s) => {
                    if cfg!(test) {
                        format!("{}/access_token", mockito::server_url())
                    } else {
                        s
                    }
                }
                Err(_e) => panic!("Env Error: REDDIT_API_ACCESS_TOKEN_URL does not exist"),
            },
            api_base_url: match env::var("REDDIT_API_BASE_URL") {
                Ok(s) => {
                    if cfg!(test) {
                        mockito::server_url()
                    } else {
                        s
                    }
                }
                Err(_e) => panic!("Env Error: REDDIT_API_BASE_URL does not exist"),
            },
        }
    }
    pub fn api_url(&self, path: &str) -> String {
        format!("{}/{}", &self.api_base_url, path)
    }
}
