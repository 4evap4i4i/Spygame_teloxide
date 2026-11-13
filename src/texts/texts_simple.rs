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