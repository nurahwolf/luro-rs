use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Destroy;

use twilight_model::gateway::payload::outgoing::UpdateVoiceState;

use crate::LuroContext;

use crate::models::LuroResponse;
use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "leave", desc = "Leave voice in a guild", dm_permission = false)]
pub struct LeaveCommand {}

#[async_trait]
impl LuroCommand for LeaveCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let guild_id = ctx.get_guild_id(&slash)?;
        let player = ctx.lavalink.player(guild_id).await.unwrap();
        player.send(Destroy::from(guild_id))?;

        slash.shard.command(&UpdateVoiceState::new(guild_id, None, false, false))?;
        slash.content("Left the channel. Goodbye!".to_owned());
        ctx.respond(&mut slash).await
    }
}
