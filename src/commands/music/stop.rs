use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Stop;

use crate::LuroContext;

use crate::models::LuroResponse;
use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "stop", desc = "Stop the currently playing track", dm_permission = false)]
pub struct StopCommand {}

#[async_trait]
impl LuroCommand for StopCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let guild_id = ctx.get_guild_id(&slash)?;

        let player = ctx.lavalink.player(guild_id).await.unwrap();
        player.send(Stop::from(guild_id))?;

        slash.content("Stopped the track!".to_owned());
        ctx.respond(&mut slash).await
    }
}
