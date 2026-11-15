pub const PLAY_PRIVATE_RU: &str = r#"Я же сказал: только в группе."#;
pub const PLAY_PRIVATE_EN: &str = r#"I said: group only."#;

pub const PLAY_GROUP_RU: &str = r#" - хост игры! Только он может её начать 
Набор в игру начался!

/getin - чтобы вступить в игру (нужно чтобы бот был активирован в личном чате).
/startgame - чтобы начать игру.
"#;
pub const PLAY_GROUP_EN: &str = r#" - is the game host! Only they can start it 
Players are being gathered!

/getin - to join the game (the bot must be activated in your private chat).
/startgame - to start the game.
"#;

pub const GETIN_SUCCES_RU: &str = r#" - добавлен в игру"#;
pub const GETIN_SUCCES_EN: &str = r#" - joined the game"#;
pub const GETIN_ERROR_RU: &str = r#" - уже в игре"#;
pub const GETIN_ERROR_EN: &str = r#" - is already in the game"#;

pub const STARTGAME_ANNOUNCE_RU: &str = r#"Йоу, игра началась игроки: {players}"#;
pub const STARTGAME_ANNOUNCE_EN: &str = r#"Yo! the game has started, players: {players}"#;

pub const STARTGAME_ERROR_RU: &str = r#"Ой-ой-ой, ошибочка, игры не будет {error}"#;
pub const STARTGAME_ERROR_EN: &str = r#"oops, an error, no game this time: {error}"#;

pub const ROLE_RU: &str = r#"Твоя роль: {role}"#;
pub const ROLE_EN: &str = r#"Your role: {role}"#;

pub const SPY_NOTICE_RU: &str = r#"Ты шпион, не выдай себя!"#;
pub const SPY_NOTICE_EN: &str = r#"You are the spy, shhh!"#;

pub const CANCEL_NOTHING_RU: &str = r#"нечего отменять"#;
pub const CANCEL_NOTHING_EN: &str = r#"nothing to cancel"#;
pub const CANCEL_SUCCESS_RU: &str = r#"игра отменена"#;
pub const CANCEL_SUCCESS_EN: &str = r#"game cancelled"#;

pub const VOTE_PROMPT_RU: &str = r#"кто шпион? жми на кнопку
проголосовали: {voted}/{total}"#;
pub const VOTE_PROMPT_EN: &str = r#"who is the spy? press a button
voted: {voted}/{total}"#;
pub const VOTE_CLOSED_RU: &str = r#"голосование окончено"#;
pub const VOTE_CLOSED_EN: &str = r#"voting is over"#;
pub const VOTE_STALE_RU: &str = r#"это голосование уже закрыто"#;
pub const VOTE_STALE_EN: &str = r#"this vote is closed"#;
pub const VOTE_NOT_IN_GAME_RU: &str = r#"ты не в игре"#;
pub const VOTE_NOT_IN_GAME_EN: &str = r#"you are not in the game"#;
pub const VOTE_ALREADY_RU: &str = r#"ты уже голосовал"#;
pub const VOTE_ALREADY_EN: &str = r#"you have already voted"#;
pub const VOTE_SELF_RU: &str = r#"за себя нельзя"#;
pub const VOTE_SELF_EN: &str = r#"you can't vote for yourself"#;

pub const VOTE_ACCEPTED_RU: &str = r#"голос принят"#;
pub const VOTE_ACCEPTED_EN: &str = r#"vote counted"#;

pub const END_NOT_HOST_RU: &str = r#"завершить может только хост"#;
pub const END_NOT_HOST_EN: &str = r#"only the host can end the game"#;

pub const FINISH_TEXT_RU: &str = r#"игра окончена

{tally}

шпион: {spy}
{verdict}"#;
pub const FINISH_TEXT_EN: &str = r#"game over

{tally}

spy: {spy}
{verdict}"#;

pub const SUSPENSE_RU: &str = "{name} — игрок с наибольшим количеством голосов и";
pub const SUSPENSE_EN: &str = "{name} got the most votes and";
pub const VERDICT_NO_VOTES_RU: &str = r#"никто не проголосовал, шпион победил"#;
pub const VERDICT_NO_VOTES_EN: &str = r#"nobody voted, the spy wins"#;
pub const VERDICT_TIE_RU: &str = r#"ничья в голосовании, шпион победил"#;
pub const VERDICT_TIE_EN: &str = r#"the vote is tied, the spy wins"#;

pub const VERDICT_SPY_CAUGHT_RU: &str = r#"{name} - шпион, вы победили"#;
pub const VERDICT_SPY_CAUGHT_EN: &str = r#"{name} is the spy, you win"#;
pub const VERDICT_WRONG_RU: &str = r#"Выбрали {name}, и... Он не шпион. Шпион победил"#;
pub const VERDICT_WRONG_EN: &str = r#"{name} was chosen and... They are not the spy. The spy wins"#;