use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Pause;

use twilight_util::builder::InteractionResponseDataBuilder;

use crate::slash::Slash;

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "pause", desc = "Pause / Unpause the current playing music", dm_permission = false)]
pub struct PauseCommand {}

#[async_trait]
impl LuroCommand for PauseCommand {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        let guild_id = ctx.interaction.guild_id.unwrap();

        let player = ctx.framework.lavalink.player(guild_id).await.unwrap();
        let paused = player.paused();
        player.send(Pause::from((guild_id, !paused)))?;

        let action = if paused { "Unpaused " } else { "Paused" };
        let _response = InteractionResponseDataBuilder::new().content(format!("{action} the track"));
        ctx.content(format!("{action} the track")).respond().await
    }
}
