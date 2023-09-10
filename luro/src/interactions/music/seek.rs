use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Seek;

use crate::interaction::LuroSlash;
use luro_model::database_driver::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "seek", desc = "Seek through the track", dm_permission = false)]
pub struct SeekCommand {
    /// Where in the track do you want to seek to (in seconds)?
    position: i64,
}

impl LuroCommand for SeekCommand {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let guild_id = ctx.interaction.guild_id.unwrap();

        let player = ctx.framework.lavalink.player(guild_id).await.unwrap();
        player.send(Seek::from((guild_id, self.position * 1000)))?;

        ctx.respond(|r| r.content(format!("Seeked to {}s", self.position))).await
    }
}
