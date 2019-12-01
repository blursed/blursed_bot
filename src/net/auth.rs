use base64;
use reqwest;
use reqwest::header::AUTHORIZATION;
use crate::shared::config::Config;

pub struct Auth<'a> {
    config: &'a Config,
}

#[derive(Deserialize, Debug)]
pub struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
}

impl<'a> Auth<'a> {
    pub fn new(config: &'a Config) -> Auth {
        Auth {
            config,
        }
    }

    pub fn get_access_token(&self) {
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "password"),
            ("username", "your_username"),
            ("password", "your_password"),
        ];
        let mut result = client
            .post("https://www.reddit.com/api/v1/access_token/.json")
            .form(&params)
            .header(
                AUTHORIZATION,
                format!("Basic {}", base64::encode("id:secret")),
            )
            .send()
            .unwrap();

        let access_token: AccessTokenResponse = result.json().unwrap();
        println!("checking result {:?}", access_token.access_token);
    }
}
