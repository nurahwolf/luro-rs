use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Volume;

use crate::interaction::LuroSlash;
use luro_model::database::drivers::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "volume", desc = "Set the volume of the player!", dm_permission = false)]
pub struct VolumeCommand {
    /// Sets the volume between 0 and 1000! 100 is the default (100% volume)
    #[command(min_value = 0, max_value = 1000)]
    volume: i64
}

impl LuroCommand for VolumeCommand {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let guild_id = ctx.interaction.guild_id.unwrap();

        let player = ctx.framework.lavalink.player(guild_id).await.unwrap();
        player.send(Volume::from((guild_id, self.volume)))?;

        ctx.respond(|r| r.content(format!("Set the volume to {}", self.volume))).await
    }
}
