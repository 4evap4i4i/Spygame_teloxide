use sqlx::PgPool;
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
    types::{ChatId, User},
    utils::command::BotCommands,
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

use rand::prelude::*;
use crate::texts::{*};

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
    /// Язык игрока - нужен, чтобы слать ему личные сообщения (роль) на его языке.
    ru: bool,
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
            ru: is_ru_user(user),
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
            bot.send_message(msg.chat.id, tr(is_ru(&msg), CANCEL_NOTHING_RU, CANCEL_NOTHING_EN)).await?;
            Ok(())
        }
    }
}

pub async fn match_started(bot: Bot, msg: Message, cmd: CommandFsmStarted, dialogue: MyDialogue, players: Vec<Player>, pool: PgPool) -> HandlerResult {
    match cmd {
        CommandFsmStarted::Getin => getin(bot, dialogue, msg, players).await,
        CommandFsmStarted::Startgame => startgame(bot, dialogue, msg, players, pool).await,
        CommandFsmStarted::Cancel => cancel(bot, dialogue, msg).await,
    }
}

pub async fn match_playin(bot: Bot, msg: Message, cmd: CommandFsmPlayin, dialogue: MyDialogue, players: Vec<Player>) -> HandlerResult {
    match cmd {
        CommandFsmPlayin::End => end(bot, dialogue, msg, players).await,
        CommandFsmPlayin::Vote(name) => vote(bot, dialogue, msg, players, name).await,
        CommandFsmPlayin::Cancel => cancel(bot, dialogue, msg).await,
    }
}

async fn play(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let ru = is_ru(&msg);

    if msg.chat.is_group() {
        let host = msg.from.as_ref().ok_or("нет отправителя")?;
        let host_name = &host.first_name;

        let players = vec![Player::new(host, true)];

        bot.send_message(
            msg.chat.id,
            format!("{host_name}{}", tr(ru, PLAY_GROUP_RU, PLAY_GROUP_EN)),
        )
        .await?;
        dialogue.update(State::Started { players }).await?;
    } else {
        bot.send_message(msg.chat.id, tr(ru, PLAY_PRIVATE_RU, PLAY_PRIVATE_EN)).await?;
    }

    Ok(())
}

async fn getin(bot: Bot, dialogue: MyDialogue, msg: Message, mut players: Vec<Player>) -> HandlerResult {
    let user = msg.from.as_ref().ok_or("нет отправителя")?;
    let player = Player::new(user, false);
    let ru = player.ru;

    if players.iter().any(|p| p.id == player.id) {
        bot.send_message(
            msg.chat.id,
            format!("{}{}", player.name, tr(ru, GETIN_ERROR_RU, GETIN_ERROR_EN)),
        )
        .await?;
        return Ok(());
    }

    let name = player.name.clone();
    players.push(player);
    dialogue.update(State::Started { players }).await?;

    bot.send_message(
        msg.chat.id,
        format!("{name}{}", tr(ru, GETIN_SUCCES_RU, GETIN_SUCCES_EN)),
    )
    .await?;

    Ok(())
}

async fn startgame(bot: Bot, dialogue: MyDialogue, msg: Message, mut players: Vec<Player>, pool: PgPool) -> HandlerResult {
    let ru = is_ru(&msg);

    let names: Vec<_> = players.iter().map(|p| p.name.as_str()).collect();
    let announcement = tr(ru, STARTGAME_ANNOUNCE_RU, STARTGAME_ANNOUNCE_EN)
        .replace("{players}", &names.join(" "));

    let spy = players.choose_mut(&mut rand::rng()).ok_or("нет игроков")?;
    spy.is_spy = true;
    let spy_id = spy.id as i64;
    let spy_ru = spy.ru;

    let role: Result<String, sqlx::Error> = sqlx::query_scalar(
        "SELECT role FROM roles WHERE id >= ( SELECT floor(random() * (max(id) - min(id) + 1)) + min(id) FROM roles ) ORDER BY id LIMIT 1;"
    )
    .fetch_one(&pool)
    .await;

    match role {
        Ok(role) => for player in &players {
            bot.send_message(
                ChatId(player.id as i64),
                tr(player.ru, ROLE_RU, ROLE_EN).replace("{role}", &role),
            )
            .await?;
        },
        Err(err) => {
            bot.send_message(
                msg.chat.id,
                tr(ru, STARTGAME_ERROR_RU, STARTGAME_ERROR_EN).replace("{error}", &err.to_string()),
            )
            .await?;

            dialogue.update(State::Start).await?;
            return Ok(());
        }
    }

    bot.send_message(msg.chat.id, announcement).await?;
    bot.send_message(ChatId(spy_id), tr(spy_ru, SPY_NOTICE_RU, SPY_NOTICE_EN)).await?;

    dialogue.update(State::Playin { players }).await?;

    Ok(())
}

/// Отмена игры из Started и Playin.
async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue.update(State::Start).await?;
    bot.send_message(msg.chat.id, tr(is_ru(&msg), CANCEL_SUCCESS_RU, CANCEL_SUCCESS_EN)).await?;
    Ok(())
}

async fn vote(bot: Bot, dialogue: MyDialogue, msg: Message, mut players: Vec<Player>, name: String) -> HandlerResult {
    let ru = is_ru(&msg);

    let name = name.trim();
    if name.is_empty() {
        bot.send_message(msg.chat.id, tr(ru, VOTE_USAGE_RU, VOTE_USAGE_EN)).await?;
        return Ok(());
    }

    let sender = msg.from.as_ref().ok_or("нет отправителя")?;

    // индекс голосующего в списке игроков
    let Some(voter_idx) = players.iter().position(|p| p.id == sender.id.0) else {
        bot.send_message(msg.chat.id, tr(ru, VOTE_NOT_IN_GAME_RU, VOTE_NOT_IN_GAME_EN)).await?;
        return Ok(());
    };

    if players[voter_idx].is_voted {
        bot.send_message(msg.chat.id, tr(ru, VOTE_ALREADY_RU, VOTE_ALREADY_EN)).await?;
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
            bot.send_message(msg.chat.id, tr(ru, VOTE_NO_SUCH_RU, VOTE_NO_SUCH_EN)).await?;
            return Ok(());
        }
        _ => {
            bot.send_message(msg.chat.id, tr(ru, VOTE_AMBIGUOUS_RU, VOTE_AMBIGUOUS_EN)).await?;
            return Ok(());
        }
    };

    if target_idx == voter_idx {
        bot.send_message(msg.chat.id, tr(ru, VOTE_SELF_RU, VOTE_SELF_EN)).await?;
        return Ok(());
    }

    players[target_idx].votes += 1;
    players[voter_idx].is_voted = true;

    let voted = players.iter().filter(|p| p.is_voted).count();
    let total = players.len();

    if voted == total {
        // проголосовали все - подводим итоги сразу
        finish_game(&bot, &dialogue, &msg, players).await
    } else {
        dialogue.update(State::Playin { players }).await?;
        bot.send_message(
            msg.chat.id,
            tr(ru, VOTE_ACCEPTED_RU, VOTE_ACCEPTED_EN)
                .replace("{name}", &sender.first_name)
                .replace("{voted}", &voted.to_string())
                .replace("{total}", &total.to_string()),
        )
        .await?;
        Ok(())
    }
}

/// Досрочное завершение голосования. Только хост.
async fn end(bot: Bot, dialogue: MyDialogue, msg: Message, players: Vec<Player>) -> HandlerResult {
    if !is_host(&players, &msg) {
        bot.send_message(msg.chat.id, tr(is_ru(&msg), END_NOT_HOST_RU, END_NOT_HOST_EN)).await?;
        return Ok(());
    }

    finish_game(&bot, &dialogue, &msg, players).await
}

/// Подсчёт голосов, объявление результата, сброс в State::Start.
async fn finish_game(bot: &Bot, dialogue: &MyDialogue, msg: &Message, players: Vec<Player>) -> HandlerResult {
    let ru = is_ru(msg);
    let spy = players.iter().find(|p| p.is_spy).ok_or("шпион не найден")?;

    let max_votes = players.iter().map(|p| p.votes).max().unwrap_or(0);
    let leaders: Vec<&Player> = players.iter().filter(|p| p.votes == max_votes).collect();

    let verdict: String = if max_votes == 0 {
        tr(ru, VERDICT_NO_VOTES_RU, VERDICT_NO_VOTES_EN).into()
    } else if leaders.len() > 1 {
        tr(ru, VERDICT_TIE_RU, VERDICT_TIE_EN).into()
    } else if leaders[0].is_spy {
        tr(ru, VERDICT_SPY_CAUGHT_RU, VERDICT_SPY_CAUGHT_EN).replace("{name}", &leaders[0].name)
    } else {
        tr(ru, VERDICT_WRONG_RU, VERDICT_WRONG_EN).replace("{name}", &leaders[0].name)
    };

    let tally = players
        .iter()
        .map(|p| format!("{}: {}", p.name, p.votes))
        .collect::<Vec<_>>()
        .join("\n");

    let text = tr(ru, FINISH_TEXT_RU, FINISH_TEXT_EN)
        .replace("{tally}", &tally)
        .replace("{spy}", &spy.name)
        .replace("{verdict}", &verdict);
    bot.send_message(msg.chat.id, text).await?;

    dialogue.update(State::Start).await?;

    Ok(())
}