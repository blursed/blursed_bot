# blursed\_bot

[![Build Status](https://travis-ci.org/blursed/blursed_bot.svg?branch=master)](https://travis-ci.org/blursed/blursed_bot)

## About

**TODO**

## Development

You will need [Rust](https://www.rust-lang.org/tools/install) installed.

Clone this repo and `cd` into the project directory to begin working on this project.

Run `cp .env.example .env`, then replace the values assigned to the environment variables with their
actual values.

To compile, run: `cargo build`

To run the test suite: `cargo test`

To execute the project in development mode: `cargo run <token>`, where `<token>` is the Bot User
OAuth Access Token from Slack. The program will then listen an respond to events in the `#testing_blursed_bot`
Slack channel.

## License

MIT
