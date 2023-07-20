use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Volume;
use twilight_model::application::interaction::Interaction;

use crate::{interactions::InteractionResponse, LuroContext, SlashResponse};

use super::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "volume", desc = "Set the volume of the player!", dm_permission = false)]
pub struct VolumeCommand {
    /// Sets the volume between 0 and 1000! 100 is the default
    #[command(min_value = 0, max_value = 1000)]
    volume: i64
}

#[async_trait]
impl LuroCommand for VolumeCommand {
    async fn run_command(self, interaction: Interaction, ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
        let ephemeral = ctx.defer_interaction(&interaction, true).await?;

        let guild_id = interaction.guild_id.unwrap();

        let player = ctx.lavalink.player(guild_id).await.unwrap();
        player.send(Volume::from((guild_id, self.volume)))?;

        Ok(InteractionResponse::Content {
            content: format!("Set the volume to {}", self.volume),
            ephemeral,
            deferred: true
        })
    }
}
