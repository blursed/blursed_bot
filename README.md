# blursed\_bot

[![Build Status](https://travis-ci.org/blursed/blursed_bot.svg?branch=master)](https://travis-ci.org/blursed/blursed_bot)

## About

**TODO**

## Development

You will need [Rust](https://www.rust-lang.org/tools/install) installed.

Clone this repo and `cd` into the project directory to begin working on this project.

To compile, run: `cargo build`

To run the test suite: `cargo test`

To execute the project in development mode: `cargo run <token>`, where `<token>` is the Bot User
OAuth Access Token from Slack. The program will then listen an respond to events in the `#testing_blursed_bot`
Slack channel.

## Deployment

Follow the documentation at https://devcenter.heroku.com. In brief, you need to install the Heroku
command line tools, and then do the following, from your local cone of this git repository:
* Add the heroku remote if you haven't done so already. (Replace `<name-of-heroku-remote>` with the
  actual heroku remote branch name in the below.)
  ```
  heroku git:remote -a <name-of-heroku-remote>
  ```
* Then to deploy `master` branch:
  ```
  git push heroku master
  ```
* Configure any required environment variables per Heroku documentation.

## License

MIT
