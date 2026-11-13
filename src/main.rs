use crate::config::Config;
use teloxide::{prelude::*, dispatching::dialogue::InMemStorage, update_listeners::webhooks};
use handlers::{*};
use std::env;

mod texts;
mod handlers;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();
    log::info!("ы");

    let config = Config::new();
    log::info!("{} , {:?}", config.render_url, env::var("PORT"));

    let handler = dptree::entry()
        .branch(Update::filter_pre_checkout_query().endpoint(pre_checkout))
        .branch(
            Update::filter_message()
                .branch(
                    dptree::entry()
                        .filter_command::<DonateCommand>()
                        .endpoint(match_donate),
                )
                .branch(Message::filter_successful_payment().endpoint(successful_payment))
                .branch(
                    dptree::entry()
                        .filter_command::<Command>()
                        .endpoint(match_command),
                )
                .enter_dialogue::<Message, InMemStorage<State>, State>()
                .branch(
                    dptree::case![State::Start]
                        .filter_command::<CommandFsmStart>()
                        .endpoint(match_start),
                )
                .branch(
                    dptree::case![State::Started { players }]
                        .filter_command::<CommandFsmStarted>()
                        .endpoint(match_started),
                )
                .branch(
                    dptree::case![State::Playin { players }]
                        .filter_command::<CommandFsmPlayin>()
                        .endpoint(match_playin),
                ),
        );

    let port: u16 = env::var("PORT")
        .expect("PORT env variable is not set")
        .parse()
        .expect("PORT env variable value is not an integer");

    let addr = ([0, 0, 0, 0], port).into();

    let host = config.render_url;
    let url = format!("{host}/webhook").parse().unwrap();

    let secret = env::var("WEBHOOK_SECRET").expect("WEBHOOK_SECRET is not set");
    let options = webhooks::Options::new(addr, url).secret_token(secret.clone());

    let listener = webhooks::axum(config.bot.clone(), options)
        .await
        .expect("ыыы");

    let pool = sqlx::PgPool::connect(&config.db_url).await?;

    Dispatcher::builder(config.bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new(), pool.clone()])
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(listener, LoggingErrorHandler::with_custom_text("Ошибка в обработчике обновлений"),)
        .await;

    pool.close().await;
    Ok(())

}