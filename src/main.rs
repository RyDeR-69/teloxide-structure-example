use teloxide::prelude::{Dispatcher, LoggingErrorHandler};
use teloxide::update_listeners::Polling;
use teloxide::{dptree, Bot};

use crate::prelude::*;

mod core;
mod error;
mod prelude;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    log::info!("Starting bot...");

    let bot = Bot::new(std::env::var("BOT_TOKEN").expect("BOT_TOKEN env var is not set"));

    let listener = Polling::builder(bot.clone())
        .timeout(std::time::Duration::from_secs(10))
        .drop_pending_updates()
        .delete_webhook()
        .await
        .build();

    let error_handler = LoggingErrorHandler::with_custom_text("An error from the dispatcher");

    Dispatcher::builder(
        bot,
        dptree::entry()
            .branch(core::admin::schema())
            .branch(core::user::schema()),
    )
    .distribution_function(|_| None::<()>)
    .default_handler(|_| async move {})
    .dependencies(dptree::deps![GlobalStorage::new()])
    .build()
    .dispatch_with_listener(listener, error_handler)
    .await;
    Ok(())
}
