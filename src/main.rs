use crate::config::Config;
use teloxide::{prelude::*};
use handlers::{match_command, Command};

mod handlers;
mod config;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();
    log::info!("ы");

    let config = Config::new();

    let handler = Update::filter_message()
            .branch(
                dptree::entry()
                    .filter_command::<Command>()
                    .endpoint(match_command),
            );


    Dispatcher::builder(config.bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;


}