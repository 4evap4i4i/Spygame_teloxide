use crate::config::Config;
use teloxide::{prelude::*, dispatching::dialogue::InMemStorage, update_listeners::webhooks};
use handlers::{*};
use std::env;

mod handlers;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();
    log::info!("ы");

    let config = Config::new();
    log::info!("{} , {:?}", config.render_url, env::var("PORT"));

    let pool = sqlx::PgPool::connect(&config.db_url).await?;

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

    let port: u16 = env::var("PORT")
        .expect("PORT env variable is not set")
        .parse()
        .expect("PORT env variable value is not an integer");

    let addr = ([0, 0, 0, 0], port).into();

    let host = config.render_url;
    let url = format!("{host}/webhook").parse().unwrap();

    let listener = webhooks::axum(config.bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("ыыы");

    Dispatcher::builder(config.bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new(), pool.clone()])
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(listener, LoggingErrorHandler::with_custom_text("Ошибка в обработчике обновлений"),)
        .await;

    pool.close().await;
    Ok(())

}