use sqlx::{PgPool, postgres::PgQueryResult};
use teloxide::{prelude::*, sugar::request::RequestReplyExt, types::InputFile, utils::command::BotCommands};

use crate::texts::{*};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command()]
    Add(String),
    #[command()]
    Start,
}



pub async fn match_command(bot: Bot, msg: Message, cmd: Command, pool: PgPool) -> HandlerResult {
    match cmd {
        Command::Add(role) => add(bot, msg, role, pool).await?,
        Command::Start => start(bot, msg).await?,
    };
    Ok(())
}

async fn start(bot: Bot, msg: Message) -> HandlerResult {
    let user = msg.from.as_ref().ok_or("Чего")?;
    let lang = user.language_code.as_deref().unwrap_or("en");

    let gif = InputFile::url("https://c.tenor.com/QqVL1Ogea6cAAAAC/tenor.gif".parse().unwrap());

    if msg.chat.is_group() {
        let user = msg.from.as_ref().ok_or("Чего")?;
        let lang = user.language_code.as_deref().unwrap_or("en");
        
        if lang.starts_with("ru") {
            bot.send_animation(msg.chat.id, gif)
                .caption(START_GROUP_RU)
                .reply_to(msg.id)
                .await?;
        } else {
            bot.send_animation(msg.chat.id, gif)
                .caption(START_GROUP_EN)
                .reply_to(msg.id)
                .await?;
        }
    }
    else {

        if lang.starts_with("ru") {
            bot.send_animation(msg.chat.id, gif)
                .caption(START_PRIVATE_RU)
                .reply_to(msg.id)
                .await?;
        } else {
            bot.send_animation(msg.chat.id, gif)
                .caption(START_PRIVATE_EN)
                .reply_to(msg.id)
                .await?;
        }
    }

    Ok(())
}

async fn add(bot: Bot, msg: Message, role: String, pool: PgPool) -> HandlerResult {
    let user = msg.from.as_ref().ok_or("Чего")?;
    let lang = user.language_code.as_deref().unwrap_or("en");
    if msg.chat.is_group() {
        if lang.starts_with("ru") {
            bot.send_message(msg.chat.id, ADD_GROUP_RU).await?;
            return Ok(());
        } else {
            bot.send_message(msg.chat.id, ADD_GROUP_EN).await?;
            return Ok(());
        }
    }

    let res: Result<String, sqlx::Error> = sqlx::query_scalar(
        "SELECT role FROM roles WHERE role = $1"
    )
    .bind(&role)
    .fetch_one(&pool)
    .await;

    match res {
        Ok(_) => {
            bot.send_message(msg.chat.id, "такая роль уже есть").await?;
        }
        Err(_) => {
            let res: Result<PgQueryResult, sqlx::Error> = sqlx::query(
                "INSERT INTO roles (role) VALUES ($1)"
            )
            .bind(&role)
            .execute(&pool)
            .await;
            
            match res {
                Ok(_) => bot.send_message(msg.chat.id, format!("роль {role} добавлена")).await?,
                Err(err) => bot.send_message(msg.chat.id, format!("ошибка {err}")).await?,
            };
        }
    }
    
    Ok(())
}