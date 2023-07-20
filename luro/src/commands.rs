use std::collections::HashMap;

use twilight_interactions::command::CreateCommand;
use twilight_model::application::command::Command;

use crate::framework::LuroFramework;

use self::{
    about::AboutCommand, boop::BoopCommand, count::CountCommand, heck::HeckCommands, hello::HelloCommand,
    moderator::ModeratorCommands, music::MusicCommands, owner::OwnerCommands, say::SayCommand, user::UserCommands
};

use std::sync::Arc;

use anyhow::bail;
use tracing::warn;
use twilight_gateway::MessageSender;
use twilight_interactions::command::CommandModel;
use twilight_model::application::interaction::{Interaction, InteractionData};

use crate::{interactions::InteractionResponse, responses::unknown_command::unknown_command};

pub mod about;
pub mod boop;
pub mod count;
pub mod heck;
pub mod hello;
pub mod moderator;
pub mod music;
pub mod owner;
pub mod say;
pub mod user;

#[derive(Default)]
pub struct Commands {
    /// Commands that are available to be registered within guilds
    pub guild_commands: HashMap<&'static str, Command>,
    /// Commands that are available to be registered globally
    pub global_commands: HashMap<&'static str, Command>
}

impl Commands {
    /// Return a default set of commands to register
    pub fn default_commands() -> Self {
        // Create the hashmaps
        let mut init = Self {
            guild_commands: Default::default(),
            global_commands: Default::default()
        };

        // Add some default commands
        init.global_commands.insert("hello", HelloCommand::create_command().into());
        init.global_commands.insert("count", CountCommand::create_command().into());
        init.global_commands.insert("say", SayCommand::create_command().into());
        init.global_commands.insert("mod", ModeratorCommands::create_command().into());
        init.global_commands.insert("music", MusicCommands::create_command().into());
        init.global_commands.insert("boop", BoopCommand::create_command().into());
        init.global_commands.insert("heck", HeckCommands::create_command().into());
        init.global_commands.insert("owner", OwnerCommands::create_command().into());
        init.global_commands.insert("about", AboutCommand::create_command().into());
        init.global_commands.insert("user", UserCommands::create_command().into());

        // Return our initialised commands
        init
    }
}

impl LuroFramework {
    /// Handle incoming command interaction.
    pub async fn handle_command(
        self: Arc<Self>,
        interaction: &Interaction,
        shard: MessageSender
    ) -> Result<InteractionResponse, anyhow::Error> {
        let data = match interaction.data.clone() {
            Some(InteractionData::ApplicationCommand(data)) => *data,
            _ => bail!("expected application command data")
        };

        Ok(match data.name.as_str() {
            "about" => AboutCommand::run(AboutCommand::from_interaction(data.into())?, interaction, &self).await?,
            "say" => SayCommand::run(SayCommand::from_interaction(data.into())?).await?,
            "user" => UserCommands::from_interaction(data.into())?.run(interaction, self).await?,
            "hello" => HelloCommand::run(interaction, &self).await?,
            "count" => CountCommand::run(CountCommand::from_interaction(data.into())?, &self).await?,
            "mod" => ModeratorCommands::run(interaction, &self, data).await?,
            "music" => MusicCommands::run(interaction, &self, data, shard).await?,
            "boop" => BoopCommand::run(interaction, &self).await?,
            "owner" => OwnerCommands::run(interaction, &self, data).await?,
            "heck" => HeckCommands::run(HeckCommands::from_interaction(data.clone().into())?, self, interaction, data).await?,
            name => {
                warn!(name = name, "received unknown command");

                unknown_command()
            }
        })
    }
}
