use anyhow::{Error, Result};
use twilight_gateway::stream::ShardRef;
use twilight_http::client::InteractionClient;
use twilight_model::{
    application::{
        command::Command,
        interaction::{application_command::CommandData, Interaction},
    },
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};

use crate::luro::Luro;

mod hello;
mod music;

fn commands() -> Vec<Command> {
    let mut cmds = Vec::new();
    cmds.extend(hello::commands());
    cmds.extend(music::commands());
    cmds
}

pub async fn register(client: &InteractionClient<'_>) -> Result<usize, Error> {
    let commands = &commands();
    client.set_global_commands(commands).await?;
    Ok(commands.len())
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
        _ => Ok(()),
    };
    if let Err(e) = res {
        tracing::error!("{e}");
    }
}

// pub async fn handle_component(
//     luro: &Luro,
//     interaction: &Interaction,
//     component: &MessageComponentInteractionData,
// ) {
//     let res = match interaction
//         .message
//         .as_ref()
//         .and_then(|m| m.interaction.as_ref())
//     {
//         Some(msg) if msg.name == "mods" => mods::list_component(ctx, interaction, component).await,
//         _ => Ok(()),
//     };
//     if let Err(e) = res {
//         tracing::error!("{e}");
//     }
// }

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
