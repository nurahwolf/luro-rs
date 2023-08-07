use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Volume;

use crate::models::LuroSlash;

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "volume", desc = "Set the volume of the player!", dm_permission = false)]
pub struct VolumeCommand {
    /// Sets the volume between 0 and 1000! 100 is the default
    #[command(min_value = 0, max_value = 1000)]
    volume: i64
}

#[async_trait]
impl LuroCommand for VolumeCommand {
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        let guild_id = ctx.interaction.guild_id.unwrap();

        let player = ctx.luro.lavalink.player(guild_id).await.unwrap();
        player.send(Volume::from((guild_id, self.volume)))?;

        ctx.content(format!("Set the volume to {}", self.volume)).respond().await
    }
}