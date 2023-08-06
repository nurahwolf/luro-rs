use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Destroy;
use twilight_model::gateway::payload::outgoing::UpdateVoiceState;

use crate::models::LuroSlash;

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "leave", desc = "Leave voice in a guild", dm_permission = false)]
pub struct LeaveCommand {}

#[async_trait]
impl LuroCommand for LeaveCommand {
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        let guild_id = ctx.interaction.guild_id.unwrap();
        let player = ctx.luro.lavalink.player(guild_id).await.unwrap();
        player.send(Destroy::from(guild_id))?;

        ctx.shard.command(&UpdateVoiceState::new(guild_id, None, false, false))?;
        ctx.content("Left the channel. Goodbye!".to_owned()).respond().await
    }
}
