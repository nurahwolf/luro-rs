

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Stop;

use crate::slash::Slash;

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "stop", desc = "Stop the currently playing track", dm_permission = false)]
pub struct StopCommand {}


impl LuroCommand for StopCommand {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        let guild_id = ctx.interaction.guild_id.unwrap();

        let player = ctx.framework.lavalink.player(guild_id).await.unwrap();
        player.send(Stop::from(guild_id))?;

        ctx.content("Stopped the track!".to_owned()).respond().await
    }
}
