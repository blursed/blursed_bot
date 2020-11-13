use crate::net::reddit_client::SearchHit;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

// See https://api.slack.com/interactivity/slash-commands for the full Slack slash command API.

// The message received from Slack.
// See https://api.slack.com/interactivity/slash-commands#app_command_handling.
#[derive(Deserialize)]
pub struct IncomingMessage {
    pub text: String,
}

// The message returned to Slack.
// See https://api.slack.com/interactivity/slash-commands#responding_to_commands.
#[derive(Serialize)]
pub struct OutgoingMessage {
    pub response_type: String,
    pub blocks: Value,
}

pub fn transform_reddit_search_hit_to_payload<'a>(
    search_hit: &'a SearchHit,
) -> Option<OutgoingMessage> {
    let payload = json!([
        {
            "type": "image",
            "image_url": &search_hit.url.to_owned(),
            "alt_text": "cute cat"
        },
        {
            "type": "section",
            "text": {
                "type": "plain_text",
                "text": &search_hit.title.to_owned(),
                "emoji": true
            }
        }
    ]);

    let message = OutgoingMessage {
        response_type: "in_channel".to_string(),
        blocks: payload.to_owned(),
    };

    Some(message)
}
