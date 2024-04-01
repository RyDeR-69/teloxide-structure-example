use teloxide::dispatching::{HandlerExt, UpdateFilterExt};
use teloxide::dptree::case;
use teloxide::prelude::{Message, Requester};
use teloxide::types::Update;
use teloxide::{dptree, Bot};

use crate::core::{AdminCommand, GlobalState};
use crate::prelude::*;

pub fn schema() -> UpdateHandler {
    let admin_id = std::env::var("ADMIN_ID")
        .expect("ADMIN_ID env var is not set")
        .parse::<u64>()
        .unwrap();

    dptree::entry().branch(
        dptree::filter(move |upd: Update| {
            upd.from()
                .map(|user| user.id.0 == admin_id)
                .unwrap_or(false)
        })
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, GlobalStorage, GlobalState>()
                .branch(
                    case![GlobalState::Idle]
                        .filter_command::<AdminCommand>()
                        .branch(case![AdminCommand::Info].endpoint(info)),
                ),
        ),
    )
}

pub async fn info(bot: Bot, message: Message) -> Result<()> {
    let bot_info = bot.get_me().await?;
    bot.send_message(message.chat.id, format!("Bot info:\n{:#?}", bot_info))
        .await?;
    Ok(())
}
