use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Seek;

use crate::LuroContext;

use crate::models::LuroResponse;
use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "seek", desc = "Seek through the track", dm_permission = false)]
pub struct SeekCommand {
    /// Where in the track do you want to seek to (in seconds)?
    position: i64
}

#[async_trait]
impl LuroCommand for SeekCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let guild_id = ctx.get_guild_id(&slash)?;

        let player = ctx.lavalink.player(guild_id).await.unwrap();
        player.send(Seek::from((guild_id, self.position * 1000)))?;

        slash.content(format!("Seeked to {}s", self.position));
        ctx.respond(&mut slash).await
    }
}
