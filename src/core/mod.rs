use teloxide::utils::command::BotCommands;

pub mod admin;
pub mod user;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "snake_case",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "start the bot")]
    Start,
    #[command(description = "display this text.")]
    Help,
}

// admin commands
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "snake_case",
    description = "Admin commands are supported:"
)]
pub enum AdminCommand {
    #[command(description = "get bot info")]
    Info,
}

#[derive(Clone, Default, Debug)]
pub enum GlobalState {
    #[default]
    Idle,
}
