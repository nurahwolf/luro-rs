use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Pause;

use crate::interaction::LuroSlash;

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "pause", desc = "Pause / Unpause the current playing music", dm_permission = false)]
pub struct PauseCommand {}

impl LuroCommand for PauseCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let guild_id = ctx.interaction.guild_id.unwrap();

        let player = ctx.framework.lavalink.player(guild_id).await.unwrap();
        let paused = player.paused();
        player.send(Pause::from((guild_id, !paused)))?;

        let actioned = if paused { "Unpaused " } else { "Paused" };
        ctx.respond(|r| r.content(format!("{actioned} the track!"))).await
    }
}
