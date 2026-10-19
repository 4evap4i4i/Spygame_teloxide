use teloxide::{dispatching::dialogue::{InMemStorage}, prelude::*, utils::command::BotCommands};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Started {
        players: Vec<Player>
    },
    Playin {
        players: Vec<Player>
    }
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum CommandFsmStarted {
    #[command()]
    Getin,
    #[command()]
    Startgame,

}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum CommandFsmStart {
    #[command()]
    Play,
    #[command()]
    Cancel,

}

#[derive(Clone)]
pub struct Player {
    id: u64,
    name: String,
    is_spy: bool,
    is_host: bool,
}

pub async fn match_start(bot: Bot, msg: Message, cmd: CommandFsmStart, dialogue: MyDialogue) -> HandlerResult {
    match cmd {
        CommandFsmStart::Play => play(bot, dialogue, msg).await,
        CommandFsmStart::Cancel => cancel(bot, dialogue, msg).await,

    }
}

pub async fn match_started(bot: Bot, msg: Message, cmd: CommandFsmStarted, dialogue: MyDialogue, players: Vec<Player>) -> HandlerResult {
    match cmd {
        CommandFsmStarted::Getin => getin(bot, dialogue, msg, players).await,
        CommandFsmStarted::Startgame => startgame(bot, dialogue, msg, players).await

    }
}

async fn play(bot: Bot, dialogue: MyDialogue, msg: Message,) -> HandlerResult {
    if msg.chat.is_group() {
        bot.send_message(msg.chat.id, "окща")
            .await?;

        let players: Vec<Player> = Vec::new();
        
        dialogue.update(State::Started { players })
            .await?
    }
    else {
        bot.send_message(msg.chat.id, "токо в группе")
            .await?;
    }

    Ok(())
}

async fn getin(bot: Bot, dialogue: MyDialogue, msg: Message, players: Vec<Player> ) -> HandlerResult {
    let mut players = players;

    let player = msg.from.unwrap();
    let player_name = player.first_name;
    let player = player.id;
    let player = player.0;

    let player = Player { id: player, name: player_name, is_spy: false, is_host: false};

    for i in &players {
        if &player.id == &i.id {
            bot.send_message(msg.chat.id, "ты уже в игре").await?;

            return Ok(());
        }
    }
    players.push(player);
    dialogue.update(State::Started { players }).await?;

    bot.send_message(msg.chat.id, "добавлен").await?;

    Ok(())
}

async fn startgame(bot: Bot, dialogue: MyDialogue, msg: Message, players: Vec<Player> ) -> HandlerResult {

    let mut announcement = String::from("йоу игра началась игроки: ");

    for i in &players {
        announcement.push_str(&format!("{} ", &i.name));
    }

    bot.send_message(msg.chat.id, announcement).await?;
    dialogue.update(State::Playin { players }).await?;

    Ok(())
}

async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::Start).await?;

    bot.send_message(msg.chat.id, "окей").await?;

    Ok(())
}