use crate::shared::config::Config;
use base64;
use reqwest;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;

pub struct Auth<'a> {
    client: &'a Client,
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
    pub fn new(client: &'a Client, config: &'a Config) -> Auth<'a> {
        Auth { client, config }
    }

    pub fn get_access_token(&self) -> String {
        let params = [
            ("grant_type", "password"),
            ("username", &self.config.username),
            ("password", &self.config.password),
        ];

        let mut result = self
            .client
            .post("https://www.reddit.com/api/v1/access_token/.json")
            .form(&params)
            .header(AUTHORIZATION, format!("Basic {:?}", self.get_basic_token()))
            .send()
            .unwrap();

        let access_token: AccessTokenResponse = result.json().unwrap();
        access_token.access_token
    }

    fn get_basic_token(&self) -> String {
        let raw_basic_token = [self.config.id.as_str(), self.config.secret.as_str()].join(":");
        base64::encode(&raw_basic_token)
    }
}
