use anyhow::Result;
use tracing::warn;
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::Interaction,
    channel::message::component::{ActionRow, Button, ButtonStyle, Component},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};

use crate::{Luro, State};

#[derive(CommandModel, CreateCommand)]
#[command(name = "boop", desc = "Boop the Bot!")]
pub struct Boop {}

pub async fn boop_command(state: State, interaction: &Interaction) -> Result<()> {
    let command_data = match Luro::get_interaction_data(interaction).await {
        Ok(ok) => ok,
        Err(why) => {
            warn!("Failed to get interaction data - {why}");
            return Ok(());
        }
    };

    let _data = match Boop::from_interaction(CommandInputData::from(*command_data)) {
        Ok(ok) => ok,
        Err(err) => {
            warn!("Failed to parse interaction data - {err}");
            Boop {}
        }
    };

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            components: Some(Vec::from([Component::ActionRow(ActionRow {
                components: Vec::from([Component::Button(Button {
                    custom_id: Some(String::from("boop")),
                    disabled: false,
                    emoji: None,
                    label: Some(String::from("Boop Me!")),
                    style: ButtonStyle::Primary,
                    url: None,
                })]),
            })])),
            content: Some(String::from("Boop Count: 0")),
            ..Default::default()
        }),
    };

    match state
        .twilight_client
        .interaction(state.data.application_info.id)
        .create_response(interaction.id, &interaction.token, &response)
        .await
    {
        Ok(ok) => ok,
        Err(_) => todo!(),
    };

    Ok(())
}

pub async fn boop_button(state: State, interaction: &Interaction) -> Result<()> {
    // Get message and parse number
    let message = interaction.message.clone().unwrap();

    let (_text, number) = message.content.split_at(12);

    let value_number = match number.parse::<i32>() {
        Ok(v) => v + 1,
        Err(_) => 0,
    };

    // Update message as interaction response
    let response = InteractionResponse {
        kind: InteractionResponseType::UpdateMessage,
        data: Some(InteractionResponseData {
            content: Some(format!("Boop Count: {}", value_number)),
            ..Default::default()
        }),
    };

    match state
        .twilight_client
        .interaction(state.data.application_info.id)
        .create_response(interaction.id, &interaction.token, &response)
        .await
    {
        Ok(ok) => ok,
        Err(_) => todo!(),
    };

    Ok(())
}
