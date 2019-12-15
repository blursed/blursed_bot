use crate::net::auth::Auth;
use crate::shared::config::Config;
use reqwest;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;

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

    pub fn get<T>(&self, path: &str, params: &T) {
        let access_token = self.auth.get_access_token();
        let result = self
            .client
            .get(self.config.api_url(path).as_str())
            .form(params)
            .header(AUTHORIZATION, format!("Bearer {:?}", access_token))
            .send()
            .unwrap();
        println!("checking get result {:?}", result)
    }
}
