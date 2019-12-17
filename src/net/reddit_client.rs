use crate::net::auth::Auth;
use crate::shared::config::Config;
use reqwest;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct RedditClient<'a> {
    auth: Auth<'a>,
    config: &'a Config,
    client: &'a Client,
}

impl<'a> RedditClient<'a> {
    pub fn new(config: &'a Config, client: &'a Client) -> RedditClient<'a> {
        let auth = Auth::new(&client, &config);

        RedditClient {
            auth,
            config,
            client,
        }
    }

    /*pub fn get<S: Deserialize<'a>, T: Serialize>(&self, path: &str, params: &T) -> S {
        let access_token = self.auth.get_access_token();
        println!(
            "checking access token {:?}",
            format!("Bearer {}", access_token)
        );
        let mut result = self
            .client
            .get(self.config.api_url(path).as_str())
            .query(params)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .unwrap();
        let response: S = result.json().unwrap();
        response
    }*/
}
