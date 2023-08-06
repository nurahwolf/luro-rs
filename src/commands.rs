use std::sync::Arc;

use tracing::error;
use tracing::warn;

use twilight_interactions::command::CreateCommand;

use self::dice::DiceCommands;
use self::info::InfoCommands;
use self::luro::LuroCommands;
use self::marry::MarryCommands;
use self::moderator::warn::ModeratorWarnCommand;
use self::{
    about::AboutCommand, base64::Base64Commands, boop::BoopCommand, count::CountCommand, heck::HeckCommands,
    hello::HelloCommand, lewd::LewdCommands, moderator::ModeratorCommands, music::MusicCommands, owner::OwnerCommands,
    say::SayCommand, story::StoryCommand, uwu::UwUCommand, wordcount::WordcountCommand
};

use twilight_model::application::interaction::InteractionData;

use crate::commands::base64::{Base64Decode, Base64Encode};
use crate::commands::heck::add::HeckAddCommand;
use crate::commands::marry::MarryNew;
use crate::framework::LuroFramework;
use crate::models::Commands;
use crate::models::LuroResponse;
use crate::traits::luro_command::LuroCommand;
use crate::BOT_NAME;

mod about;
mod base64;
mod boop;
mod count;
mod dice;
mod heck;
mod hello;
mod info;
mod lewd;
mod luro;
mod marry;
mod moderator;
mod music;
mod owner;
mod say;
mod story;
mod uwu;
mod wordcount;
// pub mod fursona;

impl Commands {
    /// Return a default set of commands to register
    pub fn default_commands() -> Self {
        // Create the hashmaps
        let mut init = Self {
            guild_commands: Default::default(),
            global_commands: Default::default()
        };

        // Add some default commands
        init.global_commands.insert("about", AboutCommand::create_command().into());
        init.global_commands.insert("hello", HelloCommand::create_command().into());
        init.global_commands.insert("count", CountCommand::create_command().into());
        init.global_commands.insert("say", SayCommand::create_command().into());
        init.global_commands.insert("mod", ModeratorCommands::create_command().into());
        init.global_commands.insert("music", MusicCommands::create_command().into());
        init.global_commands.insert("boop", BoopCommand::create_command().into());
        init.global_commands.insert("heck", HeckCommands::create_command().into());
        init.global_commands.insert("owner", OwnerCommands::create_command().into());
        init.global_commands.insert("about", AboutCommand::create_command().into());
        init.global_commands.insert("info", InfoCommands::create_command().into());
        init.global_commands.insert("lewd", LewdCommands::create_command().into());
        init.global_commands.insert("base64", Base64Commands::create_command().into());
        init.global_commands.insert("story", StoryCommand::create_command().into());
        init.global_commands.insert("uwu", UwUCommand::create_command().into());
        init.global_commands.insert("roll", DiceCommands::create_command().into());
        init.global_commands.insert("marry", MarryCommands::create_command().into());
        init.global_commands.insert(BOT_NAME, LuroCommands::create_command().into());

        init.global_commands
            .insert("wordcount", WordcountCommand::create_command().into());

        // Return our initialised commands
        init
    }
}

impl LuroFramework {
    /// Handle incoming command interaction.
    pub async fn handle_command(self: &Arc<Self>, mut slash: LuroResponse) -> anyhow::Result<()> {
        let data = match slash.interaction.data {
            Some(InteractionData::ApplicationCommand(ref data)) => *data.clone(),
            _ => {
                error!("Failing to handle interaction due to no interaction data");
                return Ok(());
            }
        };

        // TODO: CONSTANT match for bot name...
        match data.name.as_str() {
            "about" => AboutCommand::new(data).await?.run_command(self, slash).await,
            "say" => SayCommand::new(data).await?.run_command(self, slash).await,
            "info" => InfoCommands::new(data).await?.run_commands(self, slash).await,
            "hello" => HelloCommand::new(data).await?.run_command(self, slash).await,
            "count" => CountCommand::new(data).await?.run_command(self, slash).await,
            "mod" => ModeratorCommands::new(data).await?.run_commands(self, slash).await,
            "music" => MusicCommands::new(data).await?.run_commands(self, slash).await,
            "boop" => BoopCommand::new(data).await?.run_command(self, slash).await,
            "owner" => OwnerCommands::new(data).await?.run_commands(self, slash).await,
            "heck" => HeckCommands::new(data).await?.run_commands(self, slash).await,
            "lewd" => LewdCommands::new(data).await?.run_commands(self, slash).await,
            "base64" => Base64Commands::new(data).await?.run_commands(self, slash).await,
            "story" => StoryCommand::new(data).await?.run_command(self, slash).await,
            "uwu" => UwUCommand::new(data).await?.run_command(self, slash).await,
            "wordcount" => WordcountCommand::new(data).await?.run_command(self, slash).await,
            "roll" => DiceCommands::new(data).await?.run_commands(self, slash).await,
            "luro" => LuroCommands::new(data).await?.run_commands(self, slash).await,
            "marry" => MarryCommands::new(data).await?.run_commands(self, slash).await,
            _ => self.unknown_command_response(&mut slash).await
        }
    }

    /// Handle incoming component interaction
    pub async fn handle_component(self: &Arc<Self>, mut slash: LuroResponse) -> anyhow::Result<()> {
        let data = match slash.interaction.data {
            Some(InteractionData::MessageComponent(ref data)) => data.clone(),
            _ => {
                error!("Failing to handle interaction due to no interaction data");
                return Ok(());
            }
        };

        match &*data.custom_id {
            "boop" => BoopCommand::handle_component(data, self, &mut slash).await,
            "decode" => Base64Decode::handle_component(data, self, &mut slash).await,
            "encode" => Base64Encode::handle_component(data, self, &mut slash).await,
            "marry" => MarryNew::handle_component(data, self, &mut slash).await,
            "story" => StoryCommand::handle_component(data, self, &mut slash).await,
            "heck-setting" => HeckAddCommand::handle_component(data, self, &mut slash).await,
            name => {
                warn!(name = name, "received unknown component");
                self.unknown_command_response(&mut slash).await
            }
        }
    }

    /// Handle incoming modal interaction
    pub async fn handle_modal(self: &Arc<Self>, slash: LuroResponse) -> anyhow::Result<()> {
        let data = match &slash.interaction.data {
            Some(InteractionData::ModalSubmit(data)) => data.clone(),
            _ => {
                error!("Failing to handle interaction due to no interaction data");
                return Ok(());
            }
        };

        match &*data.custom_id {
            "heck-add" => HeckAddCommand::handle_model(data, self, slash).await,
            "mod-warn" => ModeratorWarnCommand::handle_model(data, self, slash).await,
            name => {
                warn!(name = name, "received unknown component");
                Ok(())
            }
        }
    }
}
