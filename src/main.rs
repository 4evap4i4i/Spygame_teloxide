mod handlers;

use teloxide::{prelude::*};
use handlers::{start, Command, echo};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();
    log::info!("ы");

    let bot = Bot::from_env();

    let handler = Update::filter_message()
            .branch(
                dptree::entry()
                    .filter_command::<Command>()
                    .endpoint(start),
            )
            .branch(
                echo,
            );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}