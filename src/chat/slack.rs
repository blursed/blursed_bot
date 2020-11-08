use serde::{Deserialize, Serialize};

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
    pub text: String,
}
