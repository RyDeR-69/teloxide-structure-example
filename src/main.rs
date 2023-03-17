mod core;
mod error;
mod prelude;

use crate::prelude::*;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dispatcher;
use teloxide::{dptree, Bot};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    log::info!("Starting bot...");

    let bot = Bot::new(""); // here is your bot token

    Dispatcher::builder(
        bot,
        dptree::entry()
            .branch(core::admin::schema())
            .branch(core::user::schema()),
    )
    .distribution_function(|_| None::<()>)
    .default_handler(|_| async move {
        // disable logging for unhandled updates
        log::warn!("Someone tried to use admin commands");
    })
    .dependencies(dptree::deps![InMemStorage::<core::GlobalState>::new()])
    .build()
    .dispatch()
    .await;
    Ok(())
}
