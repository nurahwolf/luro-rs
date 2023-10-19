use luro_framework::{ExecuteLuroCommand, CommandInteraction, responses::Response};
use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_model::{
    gateway::payload::outgoing::UpdateVoiceState,
    id::{marker::ChannelMarker, Id},
};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "join",
    desc = "Get me to join a voice channel to play some music!",
    dm_permission = false
)]
pub struct JoinCommand {
    /// The channel to join
    #[command(channel_types = "guild_voice guild_stage_voice")]
    channel: Id<ChannelMarker>,
}

impl ExecuteLuroCommand for JoinCommand {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let guild_id = match ctx.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.response_simple(Response::NotGuild).await,
        };
        ctx.shard
            .command(&UpdateVoiceState::new(guild_id, Some(self.channel), false, false))?;

        ctx.respond(|r| r.content(format!("Joined <#{}>!", self.channel))).await
    }
}
