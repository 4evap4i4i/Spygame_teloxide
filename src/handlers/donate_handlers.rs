use teloxide::{
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, LabeledPrice, Message, PreCheckoutQuery,
        TelegramTransactionId,
    },
    utils::command::BotCommands,
};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

/// Лимит из оригинала. В официальной доке не подтвердил, см. заметки ниже.
const MAX_DONATE_STARS: i64 = 2500;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum DonateCommand {
    #[command(description = "задонатить звёзды: /donate 5")]
    Donate(String),
    #[command(description = "вернуть донат: /refund <id>")]
    Refund(String),
}

pub async fn match_donate(bot: Bot, msg: Message, cmd: DonateCommand) -> HandlerResult {
    match cmd {
        DonateCommand::Donate(arg) => donate(bot, msg, arg).await,
        DonateCommand::Refund(charge_id) => refund(bot, msg, charge_id).await,
    }
}

async fn donate(bot: Bot, msg: Message, arg: String) -> HandlerResult {
    if !msg.chat.is_private() {
        bot.send_message(msg.chat.id, "Донатить можно только в личном чате с ботом.")
            .await?;
        return Ok(());
    }

    let price: i64 = match arg.trim().parse() {
        Ok(price) => price,
        Err(_) => {
            bot.send_message(
                msg.chat.id,
                "Ты неправильно ввёл сумму доната (например:\n/donate 5\nчтобы задонатить 5 звёзд)",
            )
            .await?;
            return Ok(());
        }
    };

    if price <= 0 {
        bot.send_message(msg.chat.id, "Ты чо еблан").await?;
        return Ok(());
    }
    if price > MAX_DONATE_STARS {
        bot.send_message(
            msg.chat.id,
            format!("Извини, телеграмм не даёт донатить больше {MAX_DONATE_STARS} звёзд за раз."),
        )
        .await?;
        return Ok(());
    }

    let invoice = bot
        .send_invoice(
            msg.chat.id,
            format!("Донат на сумму {price} звёзд"),
            "Донат",
            "channel_support",
            "",
            vec![LabeledPrice::new("XTR", price as u32)],
        )
        .reply_markup(pay_keyboard())
        .await;

    if let Err(e) = invoice {
        log::error!("sendInvoice failed: {e}");
        bot.send_message(msg.chat.id, "Не получилось создать счёт, попробуй позже.")
            .await?;
    }
    Ok(())
}

async fn refund(bot: Bot, msg: Message, charge_id: String) -> HandlerResult {
    let charge_id = charge_id.trim();
    if charge_id.is_empty() {
        bot.send_message(msg.chat.id, "Использование: /refund <id доната>")
            .await?;
        return Ok(());
    }

    let Some(user) = msg.from else {
        return Ok(());
    };

    if let Err(e) = bot
        .refund_star_payment(user.id, TelegramTransactionId(charge_id.to_owned()))
        .await
    {
        log::error!("refundStarPayment failed: {e}");
        bot.send_message(
            msg.chat.id,
            format!("Произошла ошибка, может ты неправильно ввёл id доната.\nОшибка: {e}"),
        )
        .await?;
    }
    Ok(())
}

pub async fn pre_checkout(bot: Bot, query: PreCheckoutQuery) -> HandlerResult {
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
    bot.send_message(
        msg.chat.id,
        format!("/refund {}", payment.telegram_payment_charge_id.0),
    )
    .await?;
    Ok(())
}

fn pay_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::pay("⭐ Донат ⭐")]])
}