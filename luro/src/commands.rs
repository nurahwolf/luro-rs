use std::str::FromStr;

use anyhow::anyhow;
use tracing::info;
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

use anyhow::bail;

use twilight_model::application::interaction::InteractionData;

use crate::commands::base64::{Base64Decode, Base64Encode};
use crate::commands::heck::add::HeckAddCommand;
use crate::commands::marry::MarryNew;
use crate::models::{Commands, CustomId, LuroSlash};
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

impl LuroSlash {
    /// Handle incoming command interaction.
    pub async fn handle_command(self) -> anyhow::Result<()> {
        let data = match self.interaction.data.clone() {
            Some(InteractionData::ApplicationCommand(data)) => *data,
            _ => bail!("expected application command data")
        };

        // TODO: CONSTANT match for bot name...
        match data.name.as_str() {
            "about" => AboutCommand::new(data).await?.run_command(self).await,
            "say" => SayCommand::new(data).await?.run_command(self).await,
            "info" => InfoCommands::new(data).await?.run_commands(self).await,
            "hello" => HelloCommand::new(data).await?.run_command(self).await,
            "count" => CountCommand::new(data).await?.run_command(self).await,
            "mod" => ModeratorCommands::new(data).await?.run_commands(self).await,
            "music" => MusicCommands::new(data).await?.run_commands(self).await,
            "boop" => BoopCommand::new(data).await?.run_command(self).await,
            "owner" => OwnerCommands::new(data).await?.run_commands(self).await,
            "heck" => HeckCommands::new(data).await?.run_commands(self).await,
            "lewd" => LewdCommands::new(data).await?.run_commands(self).await,
            "base64" => Base64Commands::new(data).await?.run_commands(self).await,
            "story" => StoryCommand::new(data).await?.run_command(self).await,
            "uwu" => UwUCommand::new(data).await?.run_command(self).await,
            "wordcount" => WordcountCommand::new(data).await?.run_command(self).await,
            "roll" => DiceCommands::new(data).await?.run_commands(self).await,
            "luro" => LuroCommands::new(data).await?.run_commands(self).await,
            "marry" => MarryCommands::new(data).await?.run_commands(self).await,
            _ => self.unknown_command_response().await
        }
    }

    /// Handle incoming component interaction
    pub async fn handle_component(self) -> anyhow::Result<()> {
        let data = match self.interaction.data {
            Some(InteractionData::MessageComponent(ref data)) => data.clone(),
            _ => return Err(anyhow!("expected message component data"))
        };

        info!(
            "Received component interaction - {} - {}",
            self.author()?.name,
            data.custom_id
        );

        match &*data.custom_id {
            "boop" => BoopCommand::handle_component(data, self).await,
            "decode" => Base64Decode::handle_component(data, self).await,
            "encode" => Base64Encode::handle_component(data, self).await,
            "marry" => MarryNew::handle_component(data, self).await,
            "story" => StoryCommand::handle_component(data, self).await,
            "heck-setting" => HeckAddCommand::handle_component(data, self).await,
            name => {
                warn!(name = name, "received unknown component");
                self.unknown_command_response().await
            }
        }
    }

    /// Handle incoming modal interaction
    pub async fn handle_modal(self) -> anyhow::Result<()> {
        let custom_id = match self.interaction.data {
            Some(InteractionData::ModalSubmit(ref data)) => CustomId::from_str(&data.custom_id)?,
            _ => return Err(anyhow!("expected modal submit data"))
        };
        let data = self.parse_modal_data(&mut self.interaction.clone())?;

        match &*custom_id.name {
            "heck-add" => HeckAddCommand::handle_model(data, self).await,
            "mod-warn" => ModeratorWarnCommand::handle_model(data, self).await,
            name => {
                warn!(name = name, "received unknown component");

                // TODO: Make this a response type.
                let embed = self
                    .default_embed()
                    .await?
                    .title("IT'S FUCKED")
                    .description("Will finish this at some point");
                self.embeds(vec![embed.build()])?.respond().await
            }
        }
    }
}
