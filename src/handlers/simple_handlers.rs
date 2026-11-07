use sqlx::{PgPool};
use teloxide::{prelude::*, utils::command::BotCommands};

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
    if msg.chat.is_group() {
        bot.send_message(msg.chat.id, "siska")
            .await?;
    }
    else {
        bot.send_message(msg.chat.id, "вне группы")
            .await?;
    }

    Ok(())
}

async fn add(bot: Bot, msg: Message, role: String, pool: PgPool) -> HandlerResult {
    if msg.chat.is_group() {
        bot.send_message(msg.chat.id, "токо в личке").await?;
        return Ok(());
    }

    let role: Result<String, sqlx::Error> = sqlx::query_scalar(
        "SELECT role FROM roles WHERE role = $1"
    )
    .bind(role)
    .fetch_one(&pool)
    .await;

    match role {
        Ok(row) => {
            bot.send_message(msg.chat.id, row).await?;
        }
        Err(row) => {
            bot.send_message(msg.chat.id, format!("ошибка {row}")).await?;
        }
    }

    pool.close().await;
    Ok(())
}