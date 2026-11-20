use teloxide::types::{Message, User};

pub fn is_ru_user(user: &User) -> bool {
    user.language_code
        .as_deref()
        .is_some_and(|l| l.starts_with("ru"))
}

pub fn is_ru(msg: &Message) -> bool {
    msg.from.as_ref().is_some_and(is_ru_user)
}

pub fn tr(ru: bool, ru_text: &'static str, en_text: &'static str) -> &'static str {
    if ru { ru_text } else { en_text }
}

pub const START_PRIVATE_RU: &str = r#"
Привет! Это бот для игры в шпиона!
    
К сожалению он работает только в групповом чате.
И тебе придётся пообщаться с другими людьми чтобы поиграть.
    
Ещё можно выйти на улицу и потрогать траву (опционально)

Зато ты можешь прописать /help чтобы увидеть весь список команд бота
    
И команду /add чтобы добавить роль в игру.
    
А также /donate + циферка - чтобы поддержать бота.
    "#;

pub const START_PRIVATE_EN: &str = r#"
Hi, this is spy game bot!
    
Unfortunately, it works only in a group.
So you must find someone to play with.
    
And touch grass.

But you can type /help to see the command list.
    
/add to add roles to the game.
    
/donate <number> - to donate stars.
    "#;

pub const START_GROUP_RU: &str = r#"
Привет, это бот для игры в шпиона!
    
Чтобы начать игру - кто-нибудь должен прописать команду /play
    
Чтобы отменить текущую игру - кто-нибудь должен прописать команду /cancel
/help - для полного списка команд.
    "#;

pub const START_GROUP_EN: &str = r#"
Hi, this is a spy game bot!
    
To start the game simply send /play to the chat.
    
/play - start new game
/cancel - cancel current game
/help - see all command
    "#;

pub const HELP_RU: &str = r#"Команды бота:

Общие:
/start - приветствие
/help - этот список

Игра (только в группе):
/play - создать игру, ты станешь хостом
/getin - вступить в игру (бот должен быть активирован в личке)
/startgame - начать игру
/vote имя - проголосовать за игрока
/end - досрочно завершить голосование (хост)
/cancel - отменить игру

Личный чат:
/add роль - добавить роль в игру
/donate число - задонатить звёзды, например /donate 5
"#;

pub const HELP_EN: &str = r#"Bot commands:

General:
/start - greeting
/help - this list

Game (group only):
/play - create a game, you become the host
/getin - join the game (the bot must be activated in your private chat)
/startgame - start the game
/vote name - vote for a player
/end - finish voting early (host)
/cancel - cancel the game

Private chat:
/add <role> - add a role to the game
/donate <number> - donate stars, e.g. /donate 5
"#;

pub const ADD_GROUP_RU: &str = r#"Добавлять роли можно только в личном чате!"#;
pub const ADD_PRIVATE_SUCCES_RU: &str = r#"Роль добавлена."#;
pub const ADD_PRIVATE_ERROR_RU: &str = r#"Извини, произошла ошибка: "#;
pub const ADD_PRIVATE_FAIL_RU: &str = r#"Такая роль уже есть"#;
pub const ADD_PRIVATE_EBLAN_RU: &str = r#"Ты чо еблан?"#;
pub const ADD_PRIVATE_EBLAN_EN: &str = r#"Are you high or something?"#;

pub const ADD_GROUP_EN: &str = r#"Roles can only be added in a private chat!"#;
pub const ADD_PRIVATE_SUCCES_EN: &str = r#"Role added."#;
pub const ADD_PRIVATE_ERROR_EN: &str = r#"Sorry, an error occurred: "#;
pub const ADD_PRIVATE_FAIL_EN: &str = r#"This role already exists"#;

pub const DONATE_GROUP_RU: &str = r#"Донатить можно только в личном чате с ботом."#;
pub const DONATE_GROUP_EN: &str = r#"You can donate only in a private chat with the bot."#;

pub const DONATE_BAD_AMOUNT_RU: &str = r#"Ты неправильно ввёл сумму доната (например:
/donate 5
чтобы задонатить 5 звёзд)"#;
pub const DONATE_BAD_AMOUNT_EN: &str = r#"You entered the donation amount incorrectly (for example:
/donate 5
to donate 5 stars)"#;

pub const DONATE_TOO_MUCH_RU: &str = r#"Извини, телеграмм не даёт донатить больше {max} звёзд за раз."#;
pub const DONATE_TOO_MUCH_EN: &str = r#"Sorry, Telegram doesn't allow donating more than {max} stars at once."#;

pub const DONATE_NON_POSITIVE_RU: &str = r#"Ты чо еблан"#;
pub const DONATE_NON_POSITIVE_EN: &str = r#"Wha da fuck is that"#;

pub const DONATE_INVOICE_TITLE_RU: &str = r#"Донат на сумму {amount} звёзд"#;
pub const DONATE_INVOICE_TITLE_EN: &str = r#"Donation of {amount} stars"#;
pub const DONATE_INVOICE_DESC_RU: &str = r#"Донат"#;
pub const DONATE_INVOICE_DESC_EN: &str = r#"Donation"#;
pub const DONATE_PAY_BUTTON_RU: &str = r#"⭐ Донат ⭐"#;
pub const DONATE_PAY_BUTTON_EN: &str = r#"⭐ Donate ⭐"#;

pub const DONATE_THANKS_RU: &str = r#"Спасибо большое за донат!

Вот команда, если захочешь их вернуть, просто скопируй и отправь сообщением:"#;
pub const DONATE_THANKS_EN: &str = r#"Thank you very much for the donation!

Here is the command if you want to get the stars back, just copy it and send as a message:"#;

pub const REFUND_OK_RU: &str = r#"Донат возвращён."#;
pub const REFUND_OK_EN: &str = r#"Donation refunded."#;

pub const REFUND_ERROR_RU: &str = r#"Произошла ошибка, может ты неправильно ввёл id доната.
Ошибка: {error}"#;
pub const REFUND_ERROR_EN: &str = r#"An error occurred, maybe you entered the donation id incorrectly.
Error: {error}"#;