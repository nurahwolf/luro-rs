use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Stop;

use crate::interaction::LuroSlash;
use luro_model::database::drivers::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq,)]
#[command(name = "stop", desc = "Stop the currently playing track", dm_permission = false)]
pub struct StopCommand {}

impl LuroCommand for StopCommand {
    async fn run_command<D: LuroDatabaseDriver,>(self, ctx: LuroSlash<D,>,) -> anyhow::Result<(),> {
        let guild_id = ctx.interaction.guild_id.unwrap();

        let player = ctx.framework.lavalink.player(guild_id,).await.unwrap();
        player.send(Stop::from(guild_id,),)?;

        ctx.respond(|r| r.content("Stopped the track!",),).await
    }
}
