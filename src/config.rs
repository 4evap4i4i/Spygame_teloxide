use std::env;

use teloxide::prelude::*;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

struct BotTexts {
    lang: &'static str,
    description: &'static str,
    short: &'static str,
}

const TEXTS: &[BotTexts] = &[
    BotTexts {
        lang: "",
        description: r#"This is spy bot to play spy game with your friends. To play you need some friends (unfortunately) in a group chat."#,
        short: "This is spy bot to play spy game with your friends.",
    },
    BotTexts {
        lang: "ru",
        description: r#"Это бот для игры в "шпиона" с друзьями! Для игры нужны друзья (к сожалению) и общая группа с ними."#,
        short: r#"Бот для игры в "шпиона" с друзьями! Для игры нужны друзья и общая группа с ними."#,
    },
];

#[derive(Debug)]
pub struct Config {
    pub bot: Bot,
    pub render_url: String,
    pub db_url: String,
}

impl Config {
    pub fn new() -> Config {
        let bot = Bot::from_env();
        let render_url =
            env::var("RENDER_EXTERNAL_URL").expect("RENDER_EXTERNAL_URL is not set");
        let db_url = env::var("DB_URL").expect("DB_URL is not set");
        Config { bot, render_url, db_url }
    }

    pub async fn setup(&self) -> HandlerResult {
        for t in TEXTS {
            let mut description = self.bot.set_my_description().description(t.description);
            let mut short = self.bot.set_my_short_description().short_description(t.short);

            if !t.lang.is_empty() {
                description = description.language_code(t.lang);
                short = short.language_code(t.lang);
            }

            description.await?;
            short.await?;
        }
        Ok(())
    }
}