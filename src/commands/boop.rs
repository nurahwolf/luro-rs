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

use crate::luro::Luro;

use super::create_response;

pub fn commands() -> Vec<Command> {
    vec![
        BoopCommand::create_command().into(),
        BoopCommandV2::create_command().into(),
    ]
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "boop", desc = "Boop the Bot!")]
pub struct BoopCommand {}

pub async fn boop_command(luro: &Luro, interaction: &Interaction) -> Result<()> {
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

    match luro
        .twilight_client
        .interaction(luro.application.id)
        .create_response(interaction.id, &interaction.token, &response)
        .await
    {
        Ok(ok) => ok,
        Err(_) => todo!(),
    };

    Ok(())
}

pub async fn boop_button(
    luro: &Luro,
    interaction: &Interaction,
    _component: &MessageComponentInteractionData,
) -> Result<()> {
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

    match luro
        .twilight_client
        .interaction(luro.application.id)
        .create_response(interaction.id, &interaction.token, &response)
        .await
    {
        Ok(ok) => ok,
        Err(_) => todo!(),
    };

    Ok(())
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "boopv2", desc = "Boop the Bot!")]
pub struct BoopCommandV2 {}

pub async fn boop_command_v2(luro: &Luro, interaction: &Interaction) -> Result<()> {
    let boop;

    {
        let mut test = luro.data.boop.write().await;
        *test = 0;
        boop = test;
    }

    let response = InteractionResponseData {
        components: Some(Vec::from([Component::ActionRow(ActionRow {
            components: Vec::from([Component::Button(Button {
                custom_id: Some(String::from("boopv2")),
                disabled: false,
                emoji: None,
                label: Some(String::from("Boop Me!")),
                style: ButtonStyle::Primary,
                url: None,
            })]),
        })])),
        content: Some(format!("Boop Count: {boop}")),
        ..Default::default()
    };

    create_response(luro, interaction, response).await?;

    Ok(())
}

pub async fn boop_button_v2(
    luro: &Luro,
    interaction: &Interaction,
    _component: &MessageComponentInteractionData,
) -> Result<()> {
    let boop: usize;

    {
        let mut test = luro.data.boop.write().await;
        *test += 1;
        boop = *test
    }

    // Update message as interaction response
    let response = InteractionResponse {
        kind: InteractionResponseType::UpdateMessage,
        data: Some(InteractionResponseData {
            content: Some(format!("Boop Count: {}", boop)),
            ..Default::default()
        }),
    };

    match luro
        .twilight_client
        .interaction(luro.application.id)
        .create_response(interaction.id, &interaction.token, &response)
        .await
    {
        Ok(ok) => ok,
        Err(_) => todo!(),
    };

    Ok(())
}
