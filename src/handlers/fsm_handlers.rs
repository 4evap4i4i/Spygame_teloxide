use teloxide::{dispatching::dialogue::{InMemStorage}, prelude::*};

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

pub async fn play(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    if msg.chat.is_group() {
        bot.send_message(msg.chat.id, "окща")
            .await?;
        let players: Vec<u64> = Vec::new();
        dialogue.update(State::Started {players})
            .await?
    }
    else {
        bot.send_message(msg.chat.id, "токо в группе")
            .await?;
    }

    Ok(())
}

pub async fn getin(bot: Bot, dialogue: MyDialogue, players: Vec<u64>, msg: Message) -> HandlerResult {
    let mut players = players;
    let player = msg.from.unwrap();
    let player_name = player.first_name;
    let player = player.id;
    let player = player.0; 

    players.push(player);
    bot.send_message(msg.chat.id, format!("Игрок {player_name} добавлен в игру!")).await?;
    dialogue.update(State::Started { players }).await?;

    Ok(())
}
/*
pub async fn players(bot: Bot, msg: Message, dialogue: MyDialogue, players: Vec<u64>) -> HandlerResult {
    let players_names = String::new();
    for i in &players {
        let player_id = &i;
        players_names.push(player_id);
    }

    Ok(())
} */