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
    blursed_url_path: String,
}

#[derive(Deserialize, Debug)]
pub struct SearchHit {
    title: String,
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct SearchResponseDataChild {
    data: SearchHit,
}

#[derive(Deserialize, Debug)]
pub struct SearchResponseData {
    after: String,
    dist: i32,
    children: Vec<SearchResponseDataChild>,
}

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    kind: String,
    data: SearchResponseData,
}

impl<'a> RedditClient<'a> {
    pub fn new(config: &'a Config, client: &'a Client) -> RedditClient<'a> {
        let auth = Auth::new(&client, &config);

        RedditClient {
            auth,
            config,
            client,
            blursed_url_path: "r/blursedimages/search".to_owned(),
        }
    }

    pub fn blursed_search<T>(&self, params: &T) -> Vec<SearchHit>
    where
        T: Serialize,
    {
        let access_token = self.auth.get_access_token();

        let mut result = self
            .client
            .get(self.config.api_url(&self.blursed_url_path).as_str())
            .query(params)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .unwrap();

        let response: SearchResponse = result.json().unwrap();

        response
            .data
            .children
            .into_iter()
            .map(|x| x.data)
            .collect::<Vec<SearchHit>>()
    }
}
