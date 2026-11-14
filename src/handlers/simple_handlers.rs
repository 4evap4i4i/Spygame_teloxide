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
    #[command()]
    Help,
}

pub async fn match_command(bot: Bot, msg: Message, cmd: Command, pool: PgPool) -> HandlerResult {
    match cmd {
        Command::Add(role) => add(bot, msg, role, pool).await?,
        Command::Start => start(bot, msg).await?,
        Command::Help => help(bot, msg).await?,
    };
    Ok(())
}

async fn start(bot: Bot, msg: Message) -> HandlerResult {
    let ru = is_ru(&msg);
    let gif = InputFile::url("https://c.tenor.com/QqVL1Ogea6cAAAAC/tenor.gif".parse().unwrap());

    let caption = if msg.chat.is_group() {
        tr(ru, START_GROUP_RU, START_GROUP_EN)
    } else {
        tr(ru, START_PRIVATE_RU, START_PRIVATE_EN)
    };

    bot.send_animation(msg.chat.id, gif)
        .caption(caption)
        .reply_to(msg.id)
        .await?;

    Ok(())
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, tr(is_ru(&msg), HELP_RU, HELP_EN))
        .await?;
    Ok(())
}

async fn add(bot: Bot, msg: Message, role: String, pool: PgPool) -> HandlerResult {
    let ru = is_ru(&msg);

    if msg.chat.is_group() {
        bot.send_message(msg.chat.id, tr(ru, ADD_GROUP_RU, ADD_GROUP_EN)).await?;
        return Ok(());
    }

    if role.is_empty() {
        bot.send_message(msg.chat.id, tr(ru, ADD_PRIVATE_EBLAN_RU, ADD_PRIVATE_EBLAN_EN)).await?;

        return Ok(());
    }

    let res: Result<String, sqlx::Error> = sqlx::query_scalar(
        "SELECT role FROM roles WHERE role = $1"
    )
    .bind(&role)
    .fetch_one(&pool)
    .await;

    match res {
        Ok(_) => {
            bot.send_message(msg.chat.id, tr(ru, ADD_PRIVATE_FAIL_RU, ADD_PRIVATE_FAIL_EN)).await?;
        }
        Err(_) => {
            let res: Result<PgQueryResult, sqlx::Error> = sqlx::query(
                "INSERT INTO roles (role) VALUES ($1)"
            )
            .bind(&role)
            .execute(&pool)
            .await;

            match res {
                Ok(_) => bot
                    .send_message(msg.chat.id, tr(ru, ADD_PRIVATE_SUCCES_RU, ADD_PRIVATE_SUCCES_EN))
                    .await?,
                Err(err) => bot
                    .send_message(
                        msg.chat.id,
                        format!("{} {err}", tr(ru, ADD_PRIVATE_ERROR_RU, ADD_PRIVATE_ERROR_EN)),
                    )
                    .await?,
            };
        }
    }

    Ok(())
}