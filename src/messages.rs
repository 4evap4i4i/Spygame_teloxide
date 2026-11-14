/// Softcoded user-facing messages for the FSM handlers and the donate handlers.
/// Keep these strings in one place so they can be adjusted without touching the
/// handler logic; the strings must stay byte-for-byte identical to before.
///
/// Messages without placeholders live as `const`s; messages with dynamic parts
/// live as formatting `fn`s. `format!` only accepts a string literal as its first
/// argument, so templates can't be passed as consts to `format!` directly.

/// Messages for the game state machine handlers (fsm_handlers.rs).
pub mod fsm {
    pub const NOTHING_TO_CANCEL: &str = "нечего отменять";
    pub const NO_SENDER: &str = "нет отправителя";
    pub const GROUP_GAME_WONT_WORK: &str = "токо в группе";
    pub const PRIVATE_NOTICE: &str = "окща";
    pub const ALREADY_IN_GAME: &str = "ты уже в игре";
    pub const ADDED: &str = "добавлен";
    pub const ROLE_ERROR: &str = "ойойойой ошибочка игры не будет {}";
    pub const SPY_IS_HOST: &str = "ты гандон";
    pub const GAME_CANCELED: &str = "игра отменена";
    pub const VOTE_HINT: &str = "напиши так: /vote имя";
    pub const NOT_IN_GAME: &str = "ты не в игре";
    pub const ALREADY_VOTED: &str = "ты уже голосовал";
    pub const NO_MATCHING_PLAYER: &str = "нет такого игрока";
    pub const NAME_CONFLICT: &str = "игроков с таким именем несколько, уточни";
    pub const CANNOT_VOTE_SELF: &str = "за себя нельзя";
    pub const END_ONLY_HOST: &str = "завершить может только хост";
    pub const SPY_NOT_FOUND: &str = "шпион не найден";
    pub const VERDICT_NO_VOTE: &str = "никто не проголосовал, шпион победил";
    pub const VERDICT_TIE: &str = "ничья в голосовании, шпион победил";
    pub fn voted_sentence(name: &str, voted: u8, total: usize) -> String {
        format!("{} проголосовал ({}/{})", name, voted, total)
    }

    pub fn started_announcement(names: &str) -> String {
        format!("йоу игра началась игроки: {}", names)
    }
    pub fn role_for_player(role: &str) -> String {
        format!("Твоя роль: {}", role)
    }
    pub fn role_error(role: &str) -> String {
        format!("ойойойой ошибочка игры не будет {}", role)
    }
    pub fn winner_sentence(name: &str) -> String {
        format!("{} - шпион, вы победили", name)
    }
    pub fn sentry_sentence(name: &str) -> String {
        format!("посадили {}, а он не шпион. шпион победил", name)
    }
    pub fn game_finalized(tally: String, spy_name: &str, verdict: String) -> String {
        format!("игра окончена\n\n{}\n\nшпион: {}\n{}", tally, spy_name, verdict)
    }
}

/// Messages for the donate handlers (donate_handlers.rs).
pub mod donate {
    pub const PRIVATE_ONLY: &str = "Донатить можно только в личном чате с ботом.";
    pub const BAD_AMOUNT_HINT: &str = "Ты неправильно ввёл сумму доната (например:\n/donate 5\nчтобы задонатить 5 звёзд)";
    pub const BAD_PRICE: &str = "Ты чо еблан";
    pub const PAYMENT_THANKS: &str = "Спасибо большое за донат!\n\nВот команда, если захочешь их вернуть, просто скопируй и отправь сообщением:";
    pub const REFUNDED: &str = "Донат возвращён.";

    pub fn max_stars_error(max_stars: i64) -> String {
        format!("Извини, телеграмм не даёт донатить больше {} звёзд за раз.", max_stars)
    }
    pub fn invoice_title(amount: u32) -> String {
        format!("Донат на сумму {} звёзд", amount)
    }
    pub fn refund_code(charge_id: &str) -> String {
        format!("<code>/refund {}</code>", charge_id)
    }
    pub fn refund_error(err: &str) -> String {
        format!("Произошла ошибка, может ты неправильно ввёл id доната.\nОшибка: {}", err)
    }
}
