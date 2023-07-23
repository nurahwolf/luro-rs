use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Seek;

use crate::responses::LuroSlash;

use super::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "seek", desc = "Seek through the track", dm_permission = false)]
pub struct SeekCommand {
    /// Where in the track do you want to seek to (in seconds)?
    position: i64
}

#[async_trait]
impl LuroCommand for SeekCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let guild_id = ctx.interaction.guild_id.unwrap();

        let player = ctx.luro.lavalink.player(guild_id).await.unwrap();
        player.send(Seek::from((guild_id, self.position * 1000)))?;

        ctx.content(format!("Seeked to {}s", self.position)).respond().await
    }
}
