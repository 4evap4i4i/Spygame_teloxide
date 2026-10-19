use teloxide::{prelude::*, utils::command::BotCommands};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command()]
    Start,
}



pub async fn match_command(bot: Bot, msg: Message, cmd: Command) -> HandlerResult {
    match cmd {
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