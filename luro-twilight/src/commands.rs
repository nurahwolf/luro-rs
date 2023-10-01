use anyhow::anyhow;
use luro_framework::command::{CreateLuroCommand, ExecuteLuroCommand};
use luro_framework::interactions::InteractionTrait;
use luro_framework::responses::Response;
use luro_framework::{CommandInteraction, ComponentInteraction, ModalInteraction};
use tracing::{info, warn};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::command::Command;
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::{Interaction, InteractionData};

#[cfg(feature = "command-about")]
mod about;
// #[cfg(feature = "command-base64")]
// mod base64;
// #[cfg(feature = "command-boop")]
// mod boop;
#[cfg(feature = "command-character")]
mod character;
// #[cfg(feature = "command-count")]
// mod count;
#[cfg(feature = "command-dice")]
mod dice;
// #[cfg(feature = "command-heck")]
// mod heck;
// #[cfg(feature = "command-hello")]
// mod hello;
#[cfg(feature = "command-info")]
mod info;
// #[cfg(feature = "command-lewd")]
// mod lewd;
// #[cfg(feature = "command-luro")]
// mod luro;
// #[cfg(feature = "command-marry")]
// mod marry;
// #[cfg(feature = "command-moderator")]
// mod moderator;
// #[cfg(feature = "command-music")]
// mod music;
#[cfg(feature = "command-owner")]
mod owner;
// #[cfg(feature = "command-ping")]
// mod ping;
// #[cfg(feature = "command-quote")]
// mod quote;
// #[cfg(feature = "command-roles")]
// mod roles;
#[cfg(feature = "command-say")]
mod say;
// #[cfg(feature = "command-story")]
// mod story;
// #[cfg(feature = "command-uwu")]
// mod uwu;
// #[cfg(feature = "command-wordcount")]
// mod wordcount;

#[derive(Debug)]
pub enum LuroCommands {
    #[cfg(feature = "command-about")]
    About(about::About),
    #[cfg(feature = "command-character")]
    Character(character::Character),
    #[cfg(feature = "command-dice")]
    Dice(dice::Dice),
    #[cfg(feature = "command-say")]
    Say(say::Say),
    #[cfg(feature = "command-info")]
    Info(info::Info),
    #[cfg(feature = "command-owner")]
    Owner(owner::Owner),
}

fn match_command<T: InteractionTrait>(ctx: &T, data: Box<CommandData>) -> anyhow::Result<LuroCommands> {
    let command = match ctx.command_name() {
        #[cfg(feature = "command-about")]
        "about" => LuroCommands::About(about::About::new(data)?),
        #[cfg(feature = "command-character")]
        "character" => LuroCommands::Character(character::Character::new(data)?),
        #[cfg(feature = "command-dice")]
        "dice" => LuroCommands::Dice(dice::Dice::new(data)?),
        #[cfg(feature = "command-say")]
        "say" => LuroCommands::Say(say::Say::new(data)?),
        #[cfg(feature = "command-info")]
        "info" => LuroCommands::Info(info::Info::new(data)?),
        #[cfg(feature = "command-owner")]
        "owner" => LuroCommands::Owner(owner::Owner::new(data)?),
        name => return Err(anyhow!("No command matching {name}")),
    };
    Ok(command)
}

pub fn default_commands() -> Vec<Command> {
    vec![
        #[cfg(feature = "command-about")]
        about::About::create_command().into(),
        #[cfg(feature = "command-character")]
        character::Character::create_command().into(),
        #[cfg(feature = "command-dice")]
        dice::Dice::create_command().into(),
        #[cfg(feature = "command-say")]
        say::Say::create_command().into(),
        #[cfg(feature = "command-info")]
        info::Info::create_command().into(),
        #[cfg(feature = "command-owner")]
        owner::Owner::create_command().into(),
    ]
}

/// Handle incoming command interaction.
pub async fn handle_command(ctx: CommandInteraction<()>) -> anyhow::Result<()> {
    info!("Received command interaction - {} - {}", ctx.author().name, ctx.data.name);
    let command = match_command(&ctx, ctx.data.clone())?;

    match command {
        #[cfg(feature = "command-about")]
        LuroCommands::About(command) => command.interaction_command(ctx).await,
        #[cfg(feature = "command-character")]
        LuroCommands::Character(command) => command.interaction_command(ctx).await,
        #[cfg(feature = "command-dice")]
        LuroCommands::Dice(command) => command.interaction_command(ctx).await,
        #[cfg(feature = "command-say")]
        LuroCommands::Say(command) => command.interaction_command(ctx).await,
        #[cfg(feature = "command-info")]
        LuroCommands::Info(command) => command.interaction_command(ctx).await,
        #[cfg(feature = "command-owner")]
        LuroCommands::Owner(command) => command.interaction_command(ctx).await,
        name => ctx.response_simple(Response::UnknownCommand(&format!("{:#?}", name))).await,
    }
}

/// Handle incoming component interaction
pub async fn handle_component(ctx: ComponentInteraction<()>) -> anyhow::Result<()> {
    info!(
        "Received component interaction - {} - {}",
        ctx.author().name,
        ctx.data.custom_id
    );

    let interaction: Interaction = match ctx
        .database
        .get_interaction_by_message_id(ctx.message.id.get() as i64)
        .await?
    {
        Some(interaction) => interaction.try_into()?,
        None => {
            warn!(ctx = ?ctx, "Attempting to handle component with an interaction that does not exist in the database");
            return Ok(());
        }
    };

    let data = match interaction.data {
        Some(InteractionData::ApplicationCommand(data)) => data,
        _ => {
            return Err(anyhow!(
                "unable to parse modal data due to not receiving ApplicationCommand data\n{:#?}",
                interaction.data
            ))
        }
    };
    let command = match_command(&ctx, data)?;

    match command {
        #[cfg(feature = "command-about")]
        LuroCommands::About(command) => command.interaction_component(ctx).await,
        #[cfg(feature = "command-character")]
        LuroCommands::Character(command) => command.interaction_component(ctx).await,
        #[cfg(feature = "command-dice")]
        LuroCommands::Dice(command) => command.interaction_component(ctx).await,
        #[cfg(feature = "command-say")]
        LuroCommands::Say(command) => command.interaction_component(ctx).await,
        #[cfg(feature = "command-info")]
        LuroCommands::Info(command) => command.interaction_component(ctx).await,
        #[cfg(feature = "command-owner")]
        LuroCommands::Owner(command) => command.interaction_component(ctx).await,
        name => ctx.response_simple(Response::UnknownCommand(&format!("{:#?}", name))).await,
    }
}

/// Handle incoming modal interaction
pub async fn handle_modal(ctx: ModalInteraction<()>) -> anyhow::Result<()> {
    info!("Received modal interaction - {} - {}", ctx.author().name, ctx.data.custom_id);

    let id = match ctx.message {
        Some(ref message) => message.id.get() as i64,
        None => {
            warn!(ctx = ?ctx, "Attempting to handle modal with an interaction that does not have a message");
            return Ok(());
        }
    };

    let interaction: Interaction = match ctx.database.get_interaction_by_message_id(id).await? {
        Some(interaction) => interaction.try_into()?,
        None => {
            warn!(ctx = ?ctx, "Attempting to handle modal with an interaction that does not exist in the database");
            return Ok(());
        }
    };

    let data = match interaction.data {
        Some(InteractionData::ApplicationCommand(data)) => data,
        _ => {
            return Err(anyhow!(
                "unable to parse modal data due to not receiving ApplicationCommand data\n{:#?}",
                interaction.data
            ))
        }
    };
    let command = match_command(&ctx, data)?;

    match command {
        #[cfg(feature = "command-about")]
        LuroCommands::About(command) => command.interaction_modal(ctx).await,
        #[cfg(feature = "command-character")]
        LuroCommands::Character(command) => command.interaction_modal(ctx).await,
        #[cfg(feature = "command-dice")]
        LuroCommands::Dice(command) => command.interaction_modal(ctx).await,
        #[cfg(feature = "command-say")]
        LuroCommands::Say(command) => command.interaction_modal(ctx).await,
        #[cfg(feature = "command-info")]
        LuroCommands::Info(command) => command.interaction_modal(ctx).await,
        #[cfg(feature = "command-owner")]
        LuroCommands::Owner(command) => command.interaction_modal(ctx).await,
        name => ctx.response_simple(Response::UnknownCommand(&format!("{:#?}", name))).await,
    }
}

/// Handle incoming autocomplete
pub async fn handle_autocomplete(ctx: CommandInteraction<()>) -> anyhow::Result<()> {
    Autocomplete::new(*ctx.data.clone())?.run(ctx).await
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
    async fn run(self, ctx: CommandInteraction<()>) -> anyhow::Result<()> {
        match self {
            #[cfg(feature = "command-character")]
            Autocomplete::Create(cmd) | Autocomplete::Icon(cmd) | Autocomplete::Send(cmd) | Autocomplete::Proxy(cmd) => {
                cmd.interaction_command(ctx).await
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
