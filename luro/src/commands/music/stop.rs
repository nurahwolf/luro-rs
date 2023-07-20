use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Stop;
use twilight_model::application::interaction::Interaction;

use crate::{interactions::InteractionResponse, LuroContext, SlashResponse};

use super::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "stop", desc = "Stop the currently playing track", dm_permission = false)]
pub struct StopCommand {}

#[async_trait]
impl LuroCommand for StopCommand {
    async fn run_command(self, interaction: Interaction, ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
        let ephemeral = ctx.defer_interaction(&interaction, true).await?;

        let guild_id = interaction.guild_id.unwrap();

        let player = ctx.lavalink.player(guild_id).await.unwrap();
        player.send(Stop::from(guild_id))?;

        Ok(InteractionResponse::Content {
            content: "Stopped the track!".to_string(),
            ephemeral,
            deferred: true
        })
    }
}
