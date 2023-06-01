use anyhow::{Error, Result};
use twilight_gateway::MessageSender;
use twilight_http::client::InteractionClient;
use twilight_model::{
    application::{
        command::Command,
        interaction::{
            application_command::CommandData, message_component::MessageComponentInteractionData,
            Interaction,
        },
    },
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
    id::Id,
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::models::luro::Luro;

mod boop;
mod heck;
mod hello;
mod moderator;
mod music;

pub fn commands() -> Vec<Command> {
    let mut cmds = Vec::new();
    cmds.extend(hello::commands());
    cmds.extend(music::commands());
    cmds.extend(boop::commands());
    cmds.extend(heck::commands());
    cmds.extend(moderator::commands());
    cmds
}

pub async fn register_global_commands(
    client: &InteractionClient<'_>,
    commands: Vec<Command>,
) -> Result<Vec<Command>, Error> {
    client.set_global_commands(&commands).await?;
    Ok(commands)
}

pub async fn handle_command(
    luro: &Luro,
    interaction: &Interaction,
    command: &CommandData,
    shard: MessageSender,
) {
    let res = match command.name.as_str() {
        "hello" => hello::hello(luro, interaction).await,
        "hellov2" => hello::hellov2(luro, interaction).await,
        "music" => music::music(luro, interaction, shard).await,
        "boop" => boop::boop_command(luro, interaction).await,
        "heck" => heck::heck(luro, interaction).await,
        "mod" => moderator::moderator(luro, interaction).await,
        _ => Ok(()),
    };
    if let Err(e) = res {
        tracing::error!("{e}");
        let _ = send_embed_v2(luro, "Error in a command".to_string(), e.to_string()).await;
    }
}

pub async fn send_embed_v2(luro: &Luro, title: String, description: String) -> Result<(), Error> {
    let embed = EmbedBuilder::default()
        .title(title)
        .description(description)
        .color(0xDABEEF)
        .build();
    let _message = luro
        .twilight_client
        .create_message(Id::new(1066690358588743760))
        .embeds(&[embed])?
        .await?;
    Ok(())
}

pub async fn handle_component(
    luro: &Luro,
    interaction: &Interaction,
    component: &MessageComponentInteractionData,
) {
    let res = match interaction
        .message
        .as_ref()
        .and_then(|m| m.interaction.as_ref())
    {
        Some(msg) => match msg.name.as_str() {
            "boop" => boop::boop_button(luro, interaction, component).await,
            _ => Ok(()),
        },
        _ => Ok(()),
    };
    if let Err(e) = res {
        tracing::error!("{e}");
    }
}

async fn create_response(
    luro: &Luro,
    interaction: &Interaction,
    data: InteractionResponseData,
) -> Result<(), Error> {
    let interaction_client: InteractionClient =
        Luro::create_interaction_client(&luro.twilight_client, &luro.application).await?;

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(data),
    };
    interaction_client
        .create_response(interaction.id, &interaction.token, &response)
        .await?;
    Ok(())
}
