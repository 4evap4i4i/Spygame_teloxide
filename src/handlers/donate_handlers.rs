use teloxide::{
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, LabeledPrice, ParseMode, PreCheckoutQuery,
        TelegramTransactionId,
    },
    utils::{command::BotCommands, html},
};

// Должен совпадать с типом ошибки в остальных хендлерах (simple_handlers, fsm_handlers),
// иначе dptree не соберёт цепочку.
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

const MAX_STARS: i64 = 2500;
const INVOICE_PAYLOAD: &str = "channel_support";

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "Донаты:")]
pub enum DonateCommand {
    #[command(description = "задонатить звёзды, например /donate 5")]
    Donate(String),
    #[command(description = "вернуть донат по id", hide)]
    Refund(String),
}

fn pay_keyboard() -> InlineKeyboardMarkup {
    // Для инвойсов первая кнопка обязана быть Pay.
    InlineKeyboardMarkup::new([[InlineKeyboardButton::pay("⭐ Донат ⭐")]])
}

pub async fn match_donate(bot: Bot, msg: Message, cmd: DonateCommand) -> HandlerResult {
    match cmd {
        DonateCommand::Donate(args) => donate(bot, msg, args).await,
        DonateCommand::Refund(args) => refund(bot, msg, args).await,
    }
}

async fn donate(bot: Bot, msg: Message, args: String) -> HandlerResult {
    if !msg.chat.is_private() {
        bot.send_message(msg.chat.id, "Донатить можно только в личном чате с ботом.")
            .await?;
        return Ok(());
    }

    // i64, чтобы отрицательные числа доходили до проверки `<= 0`, а не падали на парсинге.
    let Ok(price) = args.trim().parse::<i64>() else {
        bot.send_message(
            msg.chat.id,
            "Ты неправильно ввёл сумму доната (например:\n/donate 5\nчтобы задонатить 5 звёзд)",
        )
        .await?;
        return Ok(());
    };

    if price > MAX_STARS {
        bot.send_message(
            msg.chat.id,
            format!("Извини, телеграмм не даёт донатить больше {MAX_STARS} звёзд за раз."),
        )
        .await?;
        return Ok(());
    }
    if price <= 0 {
        bot.send_message(msg.chat.id, "Ты чо еблан").await?;
        return Ok(());
    }

    let amount = price as u32; // диапазон 1..=MAX_STARS проверен выше

    // provider_token для Stars не передаём — в teloxide-core 0.13 он опциональный.
    bot.send_invoice(
        msg.chat.id,
        format!("Донат на сумму {amount} звёзд"),
        "Донат",
        INVOICE_PAYLOAD,
        "XTR",
        [LabeledPrice::new("XTR", amount)],
    )
    .reply_markup(pay_keyboard())
    .await?;

    Ok(())
}

async fn refund(bot: Bot, msg: Message, args: String) -> HandlerResult {
    let Some(user) = msg.from.as_ref() else {
        return Ok(());
    };
    let charge_id = TelegramTransactionId::from(args.trim().to_owned());

    match bot.refund_star_payment(user.id, charge_id).await {
        Ok(_) => {
            bot.send_message(msg.chat.id, "Донат возвращён.").await?;
        }
        Err(e) => {
            log::warn!("refund_star_payment failed: {e}");
            bot.send_message(
                msg.chat.id,
                format!("Произошла ошибка, может ты неправильно ввёл id доната.\nОшибка: {e}"),
            )
            .await?;
        }
    }

    Ok(())
}

pub async fn pre_checkout(bot: Bot, query: PreCheckoutQuery) -> HandlerResult {
    // Нужно ответить в течение 10 секунд.
    bot.answer_pre_checkout_query(query.id, true).await?;
    Ok(())
}

pub async fn successful_payment(bot: Bot, msg: Message) -> HandlerResult {
    let Some(payment) = msg.successful_payment() else {
        return Ok(());
    };

    bot.send_message(
        msg.chat.id,
        "Спасибо большое за донат!\n\nВот команда, если захочешь их вернуть, просто скопируй и отправь сообщением:",
    )
    .await?;

    // <code> — чтобы по тапу текст копировался целиком. Обычный "/refund <id>" Telegram
    // подсвечивает как команду, и тап отправил бы только "/refund" без аргумента.
    bot.send_message(
        msg.chat.id,
        format!(
            "<code>/refund {}</code>",
            html::escape(&payment.telegram_payment_charge_id.0)
        ),
    )
    .parse_mode(ParseMode::Html)
    .await?;

    Ok(())
}