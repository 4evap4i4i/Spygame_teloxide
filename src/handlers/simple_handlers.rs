use teloxide::{prelude::*, utils::command::BotCommands, dispatching::dialogue::InMemStorage};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Started {
        players: Vec<u64>
    }
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command()]
    Start,
}

pub async fn match_command(bot: Bot, msg: Message, cmd: Command, dialogue: MyDialogue) -> HandlerResult {
    match cmd {
        Command::Start => start(bot, msg, dialogue).await?,
    };
    Ok(())
}

async fn start(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    if msg.chat.is_group() {
        bot.send_message(msg.chat.id, "siska")
            .await?;
        dialogue.update(State::Start)
            .await?
    }
    else {
        bot.send_message(msg.chat.id, "вне группы")
            .await?;
    }

    Ok(())
}