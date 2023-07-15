use std::collections::HashMap;

use anyhow::{bail, Error};
use tracing::warn;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::{
    command::Command,
    interaction::{application_command::CommandData, Interaction, InteractionData},
};

use crate::{
    framework::LuroFramework, interactions::InteractionResponse,
    responses::embeds::unknown_command::unknown_command,
};

use self::{
    count::CountCommand, hello::HelloCommand, moderator::ModeratorCommands, say::SayCommand,
};

pub mod count;
pub mod hello;
pub mod moderator;
pub mod say;

#[derive(Default)]
pub struct Commands {
    /// Commands that are available to be registered within guilds
    pub guild_commands: HashMap<&'static str, Command>,
    /// Commands that are available to be registered globally
    pub global_commands: HashMap<&'static str, Command>,
}

impl Commands {
    /// Return a default set of commands to register
    pub fn default_commands() -> Self {
        // Create the hashmaps
        let mut init = Self {
            guild_commands: Default::default(),
            global_commands: Default::default(),
        };

        // Add some default commands
        init.global_commands
            .insert("hello", HelloCommand::create_command().into());
        init.global_commands
            .insert("count", CountCommand::create_command().into());
        init.global_commands
            .insert("say", SayCommand::create_command().into());
        init.global_commands
            .insert("mod", ModeratorCommands::create_command().into());

        // Return our initialised commands
        init
    }
}

pub async fn handle_interaction(
    data: CommandData,
    ctx: &LuroFramework,
    interaction: Interaction,
) -> Result<InteractionResponse, Error> {
    let name = match &interaction.data {
        Some(InteractionData::ApplicationCommand(data)) => &*data.name,
        _ => bail!("expected application command data"),
    };

    match name {
        "say" => Ok(SayCommand::run(SayCommand::from_interaction(data.into())?).await?),
        "hello" => Ok(HelloCommand::execute(
            HelloCommand::from_interaction(data.into())?,
            ctx,
            interaction,
        )
        .await?),
        "count" => Ok(CountCommand::run(CountCommand::from_interaction(data.into())?, ctx).await?),
        "mod" => Ok(ModeratorCommands::handle(interaction, data, ctx).await?),
        name => {
            warn!(name = name, "received unknown command");

            Ok(unknown_command())
        }
    }
}
