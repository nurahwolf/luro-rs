use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Pause;

use crate::LuroContext;

use crate::models::LuroResponse;
use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "pause", desc = "Pause / Unpause the current playing music", dm_permission = false)]
pub struct PauseCommand {}

#[async_trait]
impl LuroCommand for PauseCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let guild_id = ctx.get_guild_id(&slash)?;

        let player = ctx.lavalink.player(guild_id).await.unwrap();
        let paused = player.paused();
        player.send(Pause::from((guild_id, !paused)))?;

        let action = if paused { "Unpaused " } else { "Paused" };
        slash.content(format!("{action} the track"));
        ctx.respond(&mut slash).await
    }
}
