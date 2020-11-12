# blursed\_bot

[![Build Status](https://travis-ci.org/blursed/blursed_bot.svg?branch=master)](https://travis-ci.org/blursed/blursed_bot)

## About

**TODO**

## Development

You will need [Rust](https://www.rust-lang.org/tools/install) installed.

Clone this repo and `cd` into the project directory to begin working on this project.

To compile, run: `cargo build`

To run the test suite: `cargo test`

## Deployment

Push to master and Travis will automatically deploy to Heroku, provided CI passes.

#### Reddit API

You can create a Reddit app at https://www.reddit.com/prefs/apps/. Currently, we are using `script app` in reddit_client.rs.
- Create a script app and copy app id and secret
- make a copy of .env.example into .env and paste app id and secret into .env
- Make sure to enter your Reddit user id and password

#### Slack API

We use Rich Message Layout to construct blursed image message
Please refer https://api.slack.com/messaging/composing/layouts to learn how to use BlockKit Message

####### Troubleshooting
- If your Reddit tokens don't work anymore, it's highly likely that the token is expired, please make a new script app from the URL above

## TODOs

- Use tokio async for reqwest
- In Reddit API apps, investigate the differences between `script` and `web` app
- Use enum to represent serde parsed object fields such as `token_type`
- `NSFW` filter from the search
- Add docs for modules like reddit_client.rs, shared and chat

## License

MIT
