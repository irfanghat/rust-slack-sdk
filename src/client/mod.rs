use dotenvy::dotenv;
use std::env;

#[derive(Debug)]
pub struct SlackClient {
    slack_bot_token: String,
}

impl Default for SlackClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SlackClient {
    pub fn new() -> Self {
        dotenv().ok();

        let slack_bot_token = env::var_os("SLACK_BOT_TOKEN")
            .unwrap()
            .into_string()
            .expect("SLACK_BOT_TOKEN is not defined in the environment.");

        Self { slack_bot_token }
    }
}
