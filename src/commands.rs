use anyhow::{Error, Result};
use twilight_gateway::stream::ShardRef;
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
};

use crate::luro::Luro;

mod boop;
mod hello;
mod music;

pub fn commands() -> Vec<Command> {
    let mut cmds = Vec::new();
    cmds.extend(hello::commands());
    cmds.extend(music::commands());
    cmds.extend(boop::commands());
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
    shard: ShardRef<'_>,
) {
    let res = match command.name.as_str() {
        "hello" => hello::hello(luro, interaction).await,
        "hellov2" => hello::hellov2(luro, interaction).await,
        "music" => music::music(luro, interaction, shard).await,
        "boop" => boop::boop_command(luro, interaction).await,
        "boopv2" => boop::boop_command_v2(luro, interaction).await,
        _ => Ok(()),
    };
    if let Err(e) = res {
        tracing::error!("{e}");
    }
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
        Some(msg) => {
            match msg.name.as_str() {
                "boop" => boop::boop_button(luro, interaction, component).await,
                "boopv2" => boop::boop_button_v2(luro, interaction, component).await,
                _ => Ok(()),
            }
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
    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(data),
    };
    luro.interaction()
        .create_response(interaction.id, &interaction.token, &response)
        .await?;
    Ok(())
}
