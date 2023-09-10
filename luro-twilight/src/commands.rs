use std::collections::HashMap;

use anyhow::anyhow;
use luro_framework::command::LuroCommandBuilder;
use luro_framework::responses::SimpleResponse;
use luro_framework::slash_command::LuroCommand;
use luro_framework::{Framework, InteractionCommand, InteractionComponent, InteractionModal, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
use tracing::{info, warn};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::application_command::CommandData;

#[cfg(feature = "command-about")]
mod about;
#[cfg(feature = "command-base64")]
mod base64;
#[cfg(feature = "command-boop")]
mod boop;
#[cfg(feature = "command-character")]
mod character;
#[cfg(feature = "command-count")]
mod count;
#[cfg(feature = "command-dice")]
mod dice;
#[cfg(feature = "command-heck")]
mod heck;
#[cfg(feature = "command-hello")]
mod hello;
#[cfg(feature = "command-info")]
mod info;
#[cfg(feature = "command-lewd")]
mod lewd;
#[cfg(feature = "command-luro")]
mod luro;
#[cfg(feature = "command-marry")]
mod marry;
#[cfg(feature = "command-moderator")]
mod moderator;
#[cfg(feature = "command-music")]
mod music;
#[cfg(feature = "command-owner")]
mod owner;
#[cfg(feature = "command-ping")]
mod ping;
#[cfg(feature = "command-quote")]
mod quote;
#[cfg(feature = "command-roles")]
mod roles;
#[cfg(feature = "command-say")]
mod say;
#[cfg(feature = "command-story")]
mod story;
#[cfg(feature = "command-uwu")]
mod uwu;
#[cfg(feature = "command-wordcount")]
mod wordcount;

pub fn default_global_commands<'a, D: LuroDatabaseDriver + 'static>() -> HashMap<&'a str, LuroCommand<D>> {
    let mut commands = HashMap::new();
    #[cfg(feature = "command-character")]
    commands.insert(character::Character::NAME, character::Character::new_command());
    #[cfg(feature = "command-dice")]
    commands.insert(dice::Dice::NAME, dice::Dice::new_command());
    #[cfg(feature = "command-marry")]
    commands.insert(marry::Marry::NAME, marry::Marry::new_command());
    #[cfg(feature = "command-ping")]
    commands.insert(ping::Ping::NAME, ping::Ping::new_command());
    #[cfg(feature = "command-quote")]
    commands.insert(quote::QuoteCommands::NAME, quote::QuoteCommands::new_command());
    #[cfg(feature = "command-roles")]
    commands.insert(roles::RoleCommands::NAME, roles::RoleCommands::new_command());
    #[cfg(feature = "command-say")]
    commands.insert(say::Say::NAME, say::Say::new_command());
    #[cfg(feature = "command-story")]
    commands.insert(story::StoryCommand::NAME, story::StoryCommand::new_command());
    #[cfg(feature = "command-uwu")]
    commands.insert(uwu::UwU::NAME, uwu::UwU::new_command());
    #[cfg(feature = "command-wordcount")]
    commands.insert(wordcount::Wordcount::NAME, wordcount::Wordcount::new_command());

    commands
}

/// Handle incoming command interaction.
pub async fn handle_command<D: LuroDatabaseDriver>(
    framework: Framework<D>,
    interaction: InteractionCommand,
) -> anyhow::Result<()> {
    info!(
        "Received command interaction - {} - {}",
        interaction.author().name,
        interaction.data.name
    );

    let command;

    {
        match framework.global_commands.lock() {
            Ok(commands) => match commands.get(&interaction.data.name) {
                Some(cmd) => command = Some(cmd.clone()),
                None => command = None,
            },
            Err(why) => {
                warn!(why = ?why, "Command mutex is poisoned");
                command = None
            }
        };
    }

    match command {
        Some(command) => (command.interaction_command)(framework.clone(), interaction).await,
        None => SimpleResponse::unknown_command(&framework, &interaction).await,
    }
}

/// Handle incoming component interaction
///
/// SAFETY: There is an unwrap here, but the type is always present on MessageComponent
/// which is the only type this function is called on
pub async fn handle_component<D: LuroDatabaseDriver>(
    framework: Framework<D>,
    interaction: InteractionComponent,
) -> anyhow::Result<()> {
    info!(
        "Received component interaction - {} - {}",
        interaction.author().name,
        interaction.data.custom_id
    );

    let command;

    {
        match framework.global_commands.lock() {
            Ok(commands) => match commands.get(&interaction.data.custom_id) {
                Some(cmd) => command = Some(cmd.clone()),
                None => command = None,
            },
            Err(why) => {
                warn!(why = ?why, "Command mutex is poisoned");
                command = None
            }
        };
    }

    match command {
        Some(command) => (command.component)(framework.clone(), interaction).await,
        None => SimpleResponse::unknown_command(&framework, &interaction).await,
    }
}

/// Handle incoming modal interaction
pub async fn handle_modal<D: LuroDatabaseDriver>(framework: Framework<D>, interaction: InteractionModal) -> anyhow::Result<()> {
    info!(
        "Received modal interaction - {} - {}",
        interaction.author().name,
        interaction.data.custom_id
    );

    let command;

    {
        match framework.global_commands.lock() {
            Ok(commands) => match commands.get(&interaction.data.custom_id) {
                Some(cmd) => command = Some(cmd.clone()),
                None => command = None,
            },
            Err(why) => {
                warn!(why = ?why, "Command mutex is poisoned");
                command = None
            }
        };
    }

    match command {
        Some(command) => (command.modal)(framework.clone(), interaction).await,
        None => SimpleResponse::unknown_command(&framework, &interaction).await,
    }
}

/// Handle incoming autocomplete
pub async fn handle_autocomplete<D: LuroDatabaseDriver>(
    ctx: Framework<D>,
    interaction: InteractionCommand,
) -> anyhow::Result<()> {
    Autocomplete::new(*interaction.data.clone())?.run(ctx, interaction).await
}

#[derive(CommandModel)]
#[command(autocomplete = true)]
enum Autocomplete {
    #[cfg(feature = "command-character")]
    #[command(name = "send")]
    Send(character::send::CharacterSendAutocomplete),
    #[cfg(feature = "command-character")]
    #[command(name = "proxy")]
    Proxy(character::send::CharacterSendAutocomplete),
    #[cfg(feature = "command-character")]
    #[command(name = "icon")]
    Icon(character::send::CharacterSendAutocomplete),
    #[cfg(feature = "command-character")]
    #[command(name = "create")]
    Create(character::send::CharacterSendAutocomplete),
}

impl Autocomplete {
    async fn run<D: LuroDatabaseDriver>(self, ctx: Framework<D>, interaction: InteractionCommand) -> anyhow::Result<()> {
        match self {
            #[cfg(feature = "command-character")]
            Autocomplete::Create(cmd) | Autocomplete::Icon(cmd) | Autocomplete::Send(cmd) | Autocomplete::Proxy(cmd) => {
                cmd.interaction_command(ctx, interaction).await
            }
        }
    }

    fn new(data: CommandData) -> anyhow::Result<Self> {
        match Self::from_interaction(data.into()) {
            Ok(ok) => Ok(ok),
            Err(why) => Err(anyhow!(
                "Got interaction data, but failed to parse it to the command type specified: {why}"
            )),
        }
    }
}
