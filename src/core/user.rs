use super::{Command, GlobalState};
use crate::prelude::*;
use teloxide::dispatching::{HandlerExt, UpdateFilterExt};
use teloxide::dptree::case;
use teloxide::prelude::{Message, Requester};
use teloxide::types::Update;
use teloxide::utils::command::BotCommands;
use teloxide::{dptree, Bot};

pub fn schema() -> UpdateHandler {
    dptree::entry().branch(
        Update::filter_message()
            .enter_dialogue::<Message, GlobalStorage, GlobalState>()
            .branch(
                case![GlobalState::Idle]
                    .filter_command::<Command>()
                    .branch(case![Command::Start].endpoint(start))
                    .branch(case![Command::Help].endpoint(help)),
            ),
    )
}

pub async fn start(bot: Bot, message: Message) -> Result<()> {
    bot.send_message(message.chat.id, "Hello World!").await?;
    Ok(())
}

pub async fn help(bot: Bot, message: Message) -> Result<()> {
    bot.send_message(message.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}
