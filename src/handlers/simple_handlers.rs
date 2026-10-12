use teloxide::{prelude::*, utils::command::BotCommands, dispatching::dialogue::InMemStorage};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command()]
    Start,
    #[command()]
    Play,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Started
}

pub async fn match_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => start(bot, msg).await?,
        Command::Play => play(bot, msg).await?
    };
    Ok(())
}

async fn start(bot: Bot, msg: Message) -> ResponseResult<()> {
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

async fn play(bot: Bot, msg: Message) -> ResponseResult<()> {
    if msg.chat.is_group() {
        bot.send_message(msg.chat.id, "окща")
            .await?;
    }
    else {
        bot.send_message(msg.chat.id, "токо в группе")
            .await?;
    }

    Ok(())
}