use base64;
use reqwest;
use reqwest::header::AUTHORIZATION;

pub struct Auth {
    username: String,
    password: String,
    id: String,
    secret: String,
}

#[derive(Deserialize, Debug)]
pub struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
}

impl Auth {
    pub fn new(username: String, password: String, id: String, secret: String) -> Auth {
        Auth {
            username,
            password,
            id,
            secret,
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
