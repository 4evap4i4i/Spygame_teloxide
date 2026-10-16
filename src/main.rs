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
            .enter_dialogue::<Message, InMemStorage<State>, State>()
/*            .branch(
                dptree::entry()
                    .filter_command::<Command>()
                    .endpoint(match_command),
            ) 
*/            .branch(
                dptree::case![State::Start]
                .endpoint(play)
            )
            .branch(
                dptree::case![State::Started { players } ]
                .endpoint(getin)
            );


    Dispatcher::builder(config.bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;


}