use std::env;

use teloxide::Bot;

#[derive(Debug)]
pub struct Config {
    pub bot: Bot,
    pub render_url: String,
    pub db_url: String,
}

impl Config {
    pub fn new() -> Config {
        let bot = Bot::from_env();
        let render_url = env::var("RENDER_EXTERNAL_URL").expect("no dick");
        let db_url = env::var("DB_URL").expect("no balls");
        Config { bot, render_url, db_url }
    }
}