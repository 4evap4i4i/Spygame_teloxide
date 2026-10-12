use teloxide::Bot;

#[derive(Debug)]
pub struct Config {
    pub bot: Bot,
}

impl Config {
    pub fn new() -> Config {
        let bot = Bot::from_env();

        Config { bot }
    }
}