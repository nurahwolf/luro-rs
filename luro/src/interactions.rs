use std::collections::HashMap;

use anyhow::anyhow;
use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::info;
use tracing::warn;
use twilight_interactions::command::CommandModel;
use twilight_interactions::command::CreateCommand;
use twilight_model::application::command::Command;
use twilight_model::application::interaction::application_command::CommandData;

use self::character::Character;
use self::dice::DiceCommands;
use self::info::InfoCommands;
use self::luro::LuroCommands;
use self::marry::MarryCommands;
use self::moderator::warn::ModeratorWarnCommand;
use self::ping::PingCommand;
use self::quote::QuoteCommands;
use self::roles::RoleCommands;
use self::{
    about::AboutCommand, base64::Base64Commands, boop::BoopCommand, count::CountCommand, heck::HeckCommands,
    hello::HelloCommand, lewd::LewdCommands, moderator::ModeratorCommands, music::MusicCommands, owner::OwnerCommands,
    say::SayCommand, story::StoryCommand, uwu::UwUCommand, wordcount::WordcountCommand
};
use crate::interaction::LuroSlash;
use crate::interactions::character::send::CharacterSendAutocomplete;
use crate::interactions::heck::add::HeckAddCommand;

use anyhow::bail;

use twilight_model::application::interaction::InteractionData;

use crate::luro_command::LuroCommand;
use crate::BOT_NAME;

mod about;
mod base64;
mod boop;
mod character;
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
mod ping;
mod quote;
mod roles;
mod say;
mod story;
mod uwu;
mod wordcount;
// pub mod fursona;

/// A simple structure containing our commands
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
        init.global_commands.insert("ping", PingCommand::create_command().into());
        init.global_commands.insert(BOT_NAME, LuroCommands::create_command().into());
        init.global_commands.insert("quote", QuoteCommands::create_command().into());
        init.global_commands.insert("roles", RoleCommands::create_command().into());
        init.global_commands.insert("character", Character::create_command().into());

        init.global_commands
            .insert("wordcount", WordcountCommand::create_command().into());

        // Return our initialised commands
        init
    }
}

impl<D: LuroDatabaseDriver> LuroSlash<D> {
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
            "info" => InfoCommands::new(data).await?.run_command(self).await,
            "hello" => HelloCommand::new(data).await?.run_command(self).await,
            "count" => CountCommand::new(data).await?.run_command(self).await,
            "mod" => ModeratorCommands::new(data).await?.run_command(self).await,
            "music" => MusicCommands::new(data).await?.run_command(self).await,
            "boop" => BoopCommand::new(data).await?.run_command(self).await,
            "owner" => OwnerCommands::new(data).await?.run_command(self).await,
            "heck" => HeckCommands::new(data).await?.run_command(self).await,
            "lewd" => LewdCommands::new(data).await?.run_command(self).await,
            "base64" => Base64Commands::new(data).await?.run_command(self).await,
            "story" => StoryCommand::new(data).await?.run_command(self).await,
            "uwu" => UwUCommand::new(data).await?.run_command(self).await,
            "wordcount" => WordcountCommand::new(data).await?.run_command(self).await,
            "dice" => DiceCommands::new(data).await?.run_command(self).await,
            "luro" => LuroCommands::new(data).await?.run_command(self).await,
            "marry" => MarryCommands::new(data).await?.run_command(self).await,
            "ping" => PingCommand::new(data).await?.run_command(self).await,
            "quote" => QuoteCommands::new(data).await?.run_command(self).await,
            "roles" => RoleCommands::new(data).await?.run_command(self).await,
            "character" => Character::new(data).await?.run_command(self).await,

            name => self.unknown_command_response_named(name).await
        }
    }

    /// Handle incoming component interaction
    ///
    /// SAFETY: There is an unwrap here, but the type is always present on MessageComponent
    /// which is the only type this function is called on
    pub async fn handle_component(self) -> anyhow::Result<()> {
        let data = self.parse_component_data(&mut self.interaction.clone())?;
        let interaction = &self.interaction;

        let original_interaction = self
            .framework
            .database
            .get_interaction(&interaction.message.as_ref().unwrap().id.to_string())
            .await?;

        let command = match original_interaction.data {
            Some(InteractionData::ApplicationCommand(data)) => *data,
            _ => {
                return Err(anyhow!(
                    "unable to parse modal data due to not receiving ApplicationCommand data\n{:#?}",
                    original_interaction.data
                ))
            }
        };

        if let Some(author) = interaction.author() {
            info!("Received component interaction - {} - {}", author.name, data.custom_id);
        }

        match &*data.custom_id {
            "character-fetish" => Character::new(command).await?.handle_component(data, self).await,
            "boop" => BoopCommand::new(command).await?.handle_component(data, self).await,
            "decode" | "encode" => Base64Commands::new(command).await?.handle_component(data, self).await,
            "marry-accept" | "marry-deny" => MarryCommands::new(command).await?.handle_component(data, self).await,
            "story" => StoryCommand::new(command).await?.handle_component(data, self).await,
            "heck-setting" => HeckCommands::new(command).await?.handle_component(data, self).await,
            "role-menu" | "rules-button" | "adult-button" | "bait-button" => {
                RoleCommands::new(command).await?.handle_component(data, self).await
            }
            name => {
                warn!(name = name, "received unknown component");
                self.unknown_command_response_named(name).await
            }
        }
    }

    /// Handle incoming modal interaction
    pub async fn handle_modal(self) -> anyhow::Result<()> {
        let data = self.parse_modal_data(&mut self.interaction.clone())?;

        // let original_interaction = self
        //     .framework
        //     .database
        //     .modal_interaction_data
        //     .get(&data.custom_id)
        //     .context("Expected to get original interaction")?
        //     .clone();

        // let command = match original_interaction.data {
        //     Some(InteractionData::ApplicationCommand(data)) => *data,
        //     _ => return Err(anyhow!("unable to parse modal data, received unknown data type"))
        // };

        match &*data.custom_id {
            "character" => Character::handle_model(data, self).await,
            "heck-add" => HeckAddCommand::handle_model(data, self).await,
            "story-add" => StoryCommand::handle_model(data, self).await,
            "mod-warn" => ModeratorWarnCommand::handle_model(data, self).await,
            "modify-embed" => OwnerCommands::handle_model(data, self).await,
            name => {
                warn!(name = name, "received unknown component");
                self.unknown_command_response_named(name).await
            }
        }
    }

    /// Handle incoming autocomplete
    pub async fn handle_autocomplete(self) -> anyhow::Result<()> {
        let data = self.parse_autocomplete_data(&self.interaction)?;
        Autocomplete::new(*data)?.run(self).await
    }
}

#[derive(CommandModel)]
#[command(autocomplete = true)]
enum Autocomplete {
    #[command(name = "send")]
    Send(CharacterSendAutocomplete),
    #[command(name = "proxy")]
    Proxy(CharacterSendAutocomplete),
    #[command(name = "icon")]
    Icon(CharacterSendAutocomplete),
    #[command(name = "create")]
    Create(CharacterSendAutocomplete)
}

impl Autocomplete {
    async fn run<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        match self {
            Autocomplete::Create(cmd) | Autocomplete::Icon(cmd) | Autocomplete::Send(cmd) | Autocomplete::Proxy(cmd) => {
                cmd.run(ctx).await
            }
        }
    }

    fn new(data: CommandData) -> anyhow::Result<Self> {
        match Self::from_interaction(data.into()) {
            Ok(ok) => Ok(ok),
            Err(why) => Err(anyhow!(
                "Got interaction data, but failed to parse it to the command type specified: {why}"
            ))
        }
    }
}
