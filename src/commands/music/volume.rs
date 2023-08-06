use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Volume;

use crate::LuroContext;

use crate::models::LuroResponse;
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
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let guild_id = ctx.get_guild_id(&slash)?;

        let player = ctx.lavalink.player(guild_id).await.unwrap();
        player.send(Volume::from((guild_id, self.volume)))?;

        slash.content(format!("Set the volume to {}", self.volume));
        ctx.respond(&mut slash).await
    }
}
