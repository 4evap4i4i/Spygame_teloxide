use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
    sugar::request::RequestReplyExt,
    types::{ChatId, User},
    utils::command::BotCommands,
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

use rand::prelude::*;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Started {
        players: Vec<Player>,
    },
    Playin {
        players: Vec<Player>,
    },
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum CommandFsmStarted {
    #[command()]
    Getin,
    #[command()]
    Startgame,
    #[command()]
    Cancel,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum CommandFsmStart {
    #[command()]
    Play,
    #[command()]
    Cancel,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum CommandFsmPlayin {
    #[command()]
    Vote(String),
    #[command()]
    End,
    #[command()]
    Cancel,
}

#[derive(Clone)]
pub struct Player {
    id: u64,
    name: String,
    is_spy: bool,
    is_host: bool,
    is_voted: bool,
    votes: u8,
}

impl Player {
    fn new(user: &User, is_host: bool) -> Self {
        Player {
            id: user.id.0,
            name: user.first_name.clone(),
            is_spy: false,
            is_host,
            is_voted: false,
            votes: 0,
        }
    }
}

/// Отправитель сообщения - хост этой игры?
fn is_host(players: &[Player], msg: &Message) -> bool {
    msg.from
        .as_ref()
        .is_some_and(|u| players.iter().any(|p| p.id == u.id.0 && p.is_host))
}

pub async fn match_start(bot: Bot, msg: Message, cmd: CommandFsmStart, dialogue: MyDialogue) -> HandlerResult {
    match cmd {
        CommandFsmStart::Play => play(bot, dialogue, msg).await,
        CommandFsmStart::Cancel => {
            bot.send_message(msg.chat.id, "нечего отменять").await?;
            Ok(())
        }
    }
}

pub async fn match_started(bot: Bot, msg: Message, cmd: CommandFsmStarted, dialogue: MyDialogue, players: Vec<Player>) -> HandlerResult {
    match cmd {
        CommandFsmStarted::Getin => getin(bot, dialogue, msg, players).await,
        CommandFsmStarted::Startgame => startgame(bot, dialogue, msg, players).await,
        CommandFsmStarted::Cancel => cancel(bot, dialogue, msg, players).await,
    }
}

pub async fn match_playin(bot: Bot, msg: Message, cmd: CommandFsmPlayin, dialogue: MyDialogue, players: Vec<Player>) -> HandlerResult {
    match cmd {
        CommandFsmPlayin::End => end(bot, dialogue, msg, players).await,
        CommandFsmPlayin::Vote(name) => vote(bot, dialogue, msg, players, name).await,
        CommandFsmPlayin::Cancel => cancel(bot, dialogue, msg, players).await,
    }
}

async fn play(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    if msg.chat.is_group() {
        // инициатор сразу становится хостом и первым игроком
        let host = msg.from.as_ref().ok_or("нет отправителя")?;
        let players = vec![Player::new(host, true)];

        bot.send_message(msg.chat.id, "окща").await?;
        dialogue.update(State::Started { players }).await?;
    } else {
        bot.send_message(msg.chat.id, "токо в группе").await?;
    }

    Ok(())
}

async fn getin(bot: Bot, dialogue: MyDialogue, msg: Message, mut players: Vec<Player>) -> HandlerResult {
    let user = msg.from.as_ref().ok_or("нет отправителя")?;
    let player = Player::new(user, false);

    if players.iter().any(|p| p.id == player.id) {
        bot.send_message(msg.chat.id, "ты уже в игре").await?;
        return Ok(());
    }

    players.push(player);
    dialogue.update(State::Started { players }).await?;

    bot.send_message(msg.chat.id, "добавлен").await?;

    Ok(())
}

async fn startgame(bot: Bot, dialogue: MyDialogue, msg: Message, mut players: Vec<Player>) -> HandlerResult {
    let names: Vec<_> = players.iter().map(|p| p.name.as_str()).collect();
    let announcement = format!("йоу игра началась игроки: {}", names.join(" "));

    let spy = players.choose_mut(&mut rand::rng()).ok_or("нет игроков")?;
    spy.is_spy = true;
    let spy_id = spy.id as i64;

    bot.send_message(msg.chat.id, announcement).await?;
    bot.send_message(ChatId(spy_id), "ты гандон").await?;

    dialogue.update(State::Playin { players }).await?;

    Ok(())
}

/// Отмена игры из Started и Playin. Только хост.
async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message, players: Vec<Player>) -> HandlerResult {
    if !is_host(&players, &msg) {
        bot.send_message(msg.chat.id, "отменить может только хост").await?;
        return Ok(());
    }

    dialogue.update(State::Start).await?;
    bot.send_message(msg.chat.id, "игра отменена").await?;

    Ok(())
}

async fn vote(bot: Bot, dialogue: MyDialogue, msg: Message, mut players: Vec<Player>, name: String) -> HandlerResult {
    let name = name.trim();
    if name.is_empty() {
        bot.send_message(msg.chat.id, "напиши так: /vote имя").await?;
        return Ok(());
    }

    let sender = msg.from.as_ref().ok_or("нет отправителя")?;

    // индекс голосующего в списке игроков
    let Some(voter_idx) = players.iter().position(|p| p.id == sender.id.0) else {
        bot.send_message(msg.chat.id, "ты не в игре").await?;
        return Ok(());
    };

    if players[voter_idx].is_voted {
        bot.send_message(msg.chat.id, "ты уже голосовал").await?;
        return Ok(());
    }

    // индексы всех игроков с таким именем (регистр не важен)
    let wanted = name.to_lowercase();
    let matches: Vec<usize> = players
        .iter()
        .enumerate()
        .filter(|(_, p)| p.name.to_lowercase() == wanted)
        .map(|(i, _)| i)
        .collect();

    let target_idx = match matches.as_slice() {
        [i] => *i,
        [] => {
            bot.send_message(msg.chat.id, "нет такого игрока").await?;
            return Ok(());
        }
        _ => {
            bot.send_message(msg.chat.id, "игроков с таким именем несколько, уточни").await?;
            return Ok(());
        }
    };

    if target_idx == voter_idx {
        bot.send_message(msg.chat.id, "за себя нельзя").await?;
        return Ok(());
    }

    players[target_idx].votes += 1;
    players[voter_idx].is_voted = true;

    let voted = players.iter().filter(|p| p.is_voted).count();
    let total = players.len();

    if voted == total {
        // проголосовали все - подводим итоги сразу
        finish_game(&bot, &dialogue, msg.chat.id, players).await
    } else {
        dialogue.update(State::Playin { players }).await?;
        bot.send_message(msg.chat.id, format!("{} проголосовал ({}/{})", sender.first_name, voted, total))
            .await?;
        Ok(())
    }
}

/// Досрочное завершение голосования. Только хост.
async fn end(bot: Bot, dialogue: MyDialogue, msg: Message, players: Vec<Player>) -> HandlerResult {
    if !is_host(&players, &msg) {
        bot.send_message(msg.chat.id, "завершить может только хост").await?;
        return Ok(());
    }

    finish_game(&bot, &dialogue, msg.chat.id, players).await
}

/// Подсчёт голосов, объявление результата, сброс в State::Start.
async fn finish_game(bot: &Bot, dialogue: &MyDialogue, chat_id: ChatId, players: Vec<Player>) -> HandlerResult {
    let spy = players.iter().find(|p| p.is_spy).ok_or("шпион не найден")?;

    let max_votes = players.iter().map(|p| p.votes).max().unwrap_or(0);
    let leaders: Vec<&Player> = players.iter().filter(|p| p.votes == max_votes).collect();

    let verdict: String = if max_votes == 0 {
        "никто не проголосовал, шпион победил".into()
    } else if leaders.len() > 1 {
        "ничья в голосовании, шпион победил".into()
    } else if leaders[0].is_spy {
        format!("{} - шпион, вы победили", leaders[0].name)
    } else {
        format!("посадили {}, а он не шпион. шпион победил", leaders[0].name)
    };

    let tally = players
        .iter()
        .map(|p| format!("{}: {}", p.name, p.votes))
        .collect::<Vec<_>>()
        .join("\n");

    let text = format!("игра окончена\n\n{}\n\nшпион: {}\n{}", tally, spy.name, verdict);
    bot.send_message(chat_id, text).await?;

    dialogue.update(State::Start).await?;

    Ok(())
}