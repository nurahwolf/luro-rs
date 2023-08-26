use anyhow::anyhow;
use luro_framework::responses::SimpleResponse;
use luro_framework::{Framework, InteractionCommand, InteractionComponent, InteractionModal, LuroInteraction};
use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::{info, warn};
use twilight_interactions::command::{ApplicationCommandData, CommandModel};
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::InteractionData;

#[cfg(feature = "command-character")]
mod character;
#[cfg(feature = "command-dice")]
mod dice;
#[cfg(feature = "command-say")]
mod say;

pub fn default_global_commands() -> Vec<ApplicationCommandData> {
    vec![
        #[cfg(feature = "command-character")]
        <character::Character as twilight_interactions::command::CreateCommand>::create_command(),
        #[cfg(feature = "command-dice")]
        <dice::Dice as twilight_interactions::command::CreateCommand>::create_command(),
        #[cfg(feature = "command-say")]
        <say::Say as twilight_interactions::command::CreateCommand>::create_command(),
    ]
}

/// Handle incoming command interaction.
pub async fn handle_command<D: LuroDatabaseDriver>(
    framework: Framework<D>,
    interaction: InteractionCommand
) -> anyhow::Result<()> {
    let data = interaction.data.clone();
    match data.name.as_str() {
        #[cfg(feature = "command-character")]
        "character" => {
            luro_framework::command::LuroCommand::interaction_command(
                <character::Character as luro_framework::command::LuroCommand>::new(data)?,
                framework,
                interaction
            )
            .await
        }
        #[cfg(feature = "command-dice")]
        "dice" => {
            luro_framework::command::LuroCommand::interaction_command(
                <dice::Dice as luro_framework::command::LuroCommand>::new(data)?,
                framework,
                interaction
            )
            .await
        }
        #[cfg(feature = "command-say")]
        "say" => {
            luro_framework::command::LuroCommand::interaction_command(
                <say::Say as luro_framework::command::LuroCommand>::new(data)?,
                framework,
                interaction
            )
            .await
        }

        name => SimpleResponse::UnknownCommand(name).respond(framework, interaction).await
    }
}

/// Handle incoming component interaction
///
/// SAFETY: There is an unwrap here, but the type is always present on MessageComponent
/// which is the only type this function is called on
pub async fn handle_component<D: LuroDatabaseDriver>(
    ctx: Framework<D>,
    interaction: InteractionComponent
) -> anyhow::Result<()> {
    let mut message = interaction.message.clone();
    let mut original_interaction = interaction.original.clone();
    let mut new_id = true;

    while new_id {
        original_interaction = ctx.database.get_interaction(&message.id.to_string()).await?;

        new_id = match original_interaction.message {
            Some(ref new_message) => {
                message = new_message.clone();
                true
            }
            None => false
        }
    }

    let command = match original_interaction.data {
        Some(InteractionData::ApplicationCommand(ref data)) => data.clone(),
        _ => {
            return Err(anyhow!(
                "unable to parse modal data due to not receiving ApplicationCommand data\n{:#?}",
                interaction.data
            ))
        }
    };

    if let Some(author) = interaction.author() {
        info!(
            "Received component interaction - {} - {}",
            author.name, interaction.data.custom_id
        );
    }

    match interaction.data.custom_id.as_str() {
        #[cfg(feature = "command-character")]
        "character" => {
            luro_framework::command::LuroCommand::handle_component(
                <character::Character as luro_framework::command::LuroCommand>::new(command)?,
                ctx,
                interaction
            )
            .await
        }
        name => {
            warn!(name = name, "received unknown component");
            // self.unknown_command_response_named(name).await
            Ok(())
        }
    }
}

/// Handle incoming modal interaction
pub async fn handle_modal<D: LuroDatabaseDriver>(ctx: Framework<D>, interaction: InteractionModal) -> anyhow::Result<()> {
    if let Some(author) = interaction.author() {
        info!(
            "Received component interaction - {} - {}",
            author.name, interaction.data.custom_id
        );
    }

    match interaction.data.custom_id.as_str() {
        #[cfg(feature = "command-character")]
        "character" => <character::Character as luro_framework::command::LuroCommand>::handle_modal(ctx, interaction).await,
        name => {
            warn!(name = name, "received unknown component");
            // ctx.unknown_command_response_named(name).await
            Ok(())
        }
    }
}

/// Handle incoming autocomplete
pub async fn handle_autocomplete<D: LuroDatabaseDriver>(
    ctx: Framework<D>,
    interaction: InteractionCommand
) -> anyhow::Result<()> {
    Autocomplete::new(*interaction.data.clone())?.run(ctx, interaction).await
}

#[derive(CommandModel)]
#[command(autocomplete = true)]
enum Autocomplete {
    #[command(name = "send")]
    Send(character::send::CharacterSendAutocomplete),
    #[command(name = "proxy")]
    Proxy(character::send::CharacterSendAutocomplete),
    #[command(name = "icon")]
    Icon(character::send::CharacterSendAutocomplete),
    #[command(name = "create")]
    Create(character::send::CharacterSendAutocomplete)
}

impl Autocomplete {
    async fn run<D: LuroDatabaseDriver>(self, ctx: Framework<D>, interaction: InteractionCommand) -> anyhow::Result<()> {
        match self {
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
            ))
        }
    }
}
