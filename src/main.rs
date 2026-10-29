use crate::config::Config;
use teloxide::{prelude::*, dispatching::dialogue::InMemStorage};
use handlers::{*};

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
            ) 
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(
                dptree::case![State::Start]
                    .filter_command::<CommandFsmStart>()
                    .endpoint(match_start)
            )
            .branch(
                dptree::case![State::Started { players }]
                    .filter_command::<CommandFsmStarted>()
                    .endpoint(match_started)
            )
            .branch(
                dptree::case![State::Playin { players }]
                    .filter_command::<CommandFsmPlayin>()
                    .endpoint(match_playin)
            );


    Dispatcher::builder(config.bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;


}