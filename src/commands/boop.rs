use anyhow::Result;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::{
        command::Command,
        interaction::{message_component::MessageComponentInteractionData, Interaction},
    },
    channel::message::component::{ActionRow, Button, ButtonStyle, Component},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};

use crate::models::luro::Luro;

pub fn commands() -> Vec<Command> {
    vec![BoopCommand::create_command().into()]
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "boop", desc = "Boop the Bot!")]
pub struct BoopCommand {}

pub async fn boop_command(luro: &Luro, interaction: &Interaction) -> Result<()> {
    let interaction_client =
        Luro::create_interaction_client(&luro.twilight_client, &luro.application).await?;

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

    interaction_client
        .create_response(interaction.id, &interaction.token, &response)
        .await?;

    Ok(())
}

pub async fn boop_button(
    luro: &Luro,
    interaction: &Interaction,
    _component: &MessageComponentInteractionData,
) -> Result<()> {
    let interaction_client =
        Luro::create_interaction_client(&luro.twilight_client, &luro.application).await?;
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

    match interaction_client
        .create_response(interaction.id, &interaction.token, &response)
        .await
    {
        Ok(ok) => ok,
        Err(_) => todo!(),
    };

    Ok(())
}
