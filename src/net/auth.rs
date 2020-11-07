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
            .post(&self.config.access_token_url)
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

#[cfg(test)]
mod test {
    use crate::net::auth::Auth;
    use crate::shared::config::Config;
    use reqwest::header::AUTHORIZATION;
    use serde_json::json;

    #[test]
    fn get_access_token_successfully() {
        let client = reqwest::Client::new();
        let config = Config::load();
        let auth = Auth::new(&client, &config);

        let access_token = "access token";
        let response_payload = json!({
           "access_token": access_token,
           "token_type": "type",
           "expires_in": 123,
           "scope": "test",
        });

        let m = mock("POST", "/access_token")
            .match_header(AUTHORIZATION.as_str(), Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(response_payload.to_string())
            .create();

        assert_eq!(auth.get_access_token(), access_token)
    }
}
