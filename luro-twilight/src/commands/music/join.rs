use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_model::{
    gateway::payload::outgoing::UpdateVoiceState,
    id::{marker::ChannelMarker, Id},
};

use crate::interaction::LuroSlash;
use luro_model::database::drivers::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;
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

impl LuroCommand for JoinCommand {
    async fn run_command(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let guild_id = ctx.interaction.guild_id.unwrap();

        ctx.shard
            .command(&UpdateVoiceState::new(guild_id, Some(self.channel), false, false))?;

        ctx.respond(|r| r.content(format!("Joined <#{}>!", self.channel))).await
    }
}
