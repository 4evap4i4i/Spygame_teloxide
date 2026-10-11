use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command()]
    Start,
}

pub async fn start(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, format!("нет"))
                .await?
        },
    };
    Ok(())
}

pub async fn echo(bot: Bot, msg: Message) -> ResponseResult<()> {
    let echo = msg.text().unwrap_or("чо");
    bot.send_message(msg.chat.id, echo).await?;
    Ok(())
}