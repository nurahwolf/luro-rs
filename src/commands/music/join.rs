use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_model::{
    gateway::payload::outgoing::UpdateVoiceState,
    id::{marker::ChannelMarker, Id}
};

use crate::{models::LuroResponse, LuroContext};

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "join",
    desc = "Get me to join a voice channel to play some music!",
    dm_permission = false
)]
pub struct JoinCommand {
    /// The channel to join
    #[command(channel_types = "guild_voice guild_stage_voice")]
    channel: Id<ChannelMarker>
}

#[async_trait]
impl LuroCommand for JoinCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let guild_id = ctx.get_guild_id(&slash)?;

        slash
            .shard
            .command(&UpdateVoiceState::new(guild_id, Some(self.channel), false, false))?;

        slash.content(format!("Joined <#{}>!", self.channel));
        ctx.respond(&mut slash).await
    }
}
