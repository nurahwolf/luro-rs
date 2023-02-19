use anyhow::Result;
use tracing::warn;
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::{application_command::InteractionChannel, Interaction},
    channel::message::MessageFlags,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
    id::{marker::ChannelMarker, Id},
};

use crate::Luro;

#[derive(CommandModel, CreateCommand)]
#[command(name = "say", desc = "Get Luro to say a message")]
pub struct SayCommand {
    /// Message to send
    message: String,
    /// The channel returned in the message interaction
    interaction_channel: InteractionChannel,
    /// The channel to send the message
    channel: Option<Id<ChannelMarker>>,
}

pub async fn say_command<'a>(luro: &Luro, interaction: &Interaction) -> Result<()> {
    let command_data = match Luro::get_interaction_data(interaction).await {
        Ok(ok) => ok,
        Err(why) => {
            warn!("Failed to get interaction data - {why}");
            return Ok(());
        }
    };

    let interaction_data = match SayCommand::from_interaction(CommandInputData::from(*command_data))
    {
        Ok(ok) => ok,
        Err(why) => {
            warn!("Failed to parse interaction data - {why}");
            return Ok(());
        }
    };

    let channel_to_send = match interaction_data.channel {
        Some(channel) => channel,
        None => interaction_data.interaction_channel.id,
    };

    match luro
        .http
        .create_message(channel_to_send)
        .content(&interaction_data.message)
    {
        Ok(ok) => ok,
        Err(why) => {
            warn!("Failed to send message because: {why}");
            return Ok(());
        }
    }
    .await?;

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            content: Some("Sent!".to_string()),
            flags: Some(MessageFlags::EPHEMERAL),
            ..Default::default()
        }),
    };

    match luro
        .http
        .interaction(luro.application_id)
        .create_response(interaction.id, &interaction.token, &response)
        .await
    {
        Ok(ok) => ok,
        Err(why) => {
            warn!("Failed to send interaction: {why}");
            return Ok(());
        }
    };

    Ok(())
}
