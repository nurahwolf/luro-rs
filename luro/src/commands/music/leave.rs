use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Destroy;
use twilight_model::{
    application::interaction::Interaction, gateway::payload::outgoing::UpdateVoiceState,
};

use crate::{framework::LuroFramework, interactions::InteractionResponse};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "leave", desc = "Leave voice in a guild", dm_permission = false)]
pub struct LeaveCommand {}

pub async fn leave(
    ctx: &LuroFramework,
    interaction: &Interaction,
    shard: MessageSender,
) -> anyhow::Result<InteractionResponse> {
    tracing::debug!(
        "leave command in channel {} by {}",
        interaction.channel.clone().unwrap().name.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let guild_id = interaction.guild_id.unwrap();
    let player = ctx.lavalink.player(guild_id).await.unwrap();
    player.send(Destroy::from(guild_id))?;

    shard.command(&UpdateVoiceState::new(guild_id, None, false, false))?;
    Ok(InteractionResponse::Text {
        content: "Left the channel. Goodbye!".to_string(),
        components: None,
        ephemeral: true,
    })
}
