use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Destroy;
use twilight_model::gateway::payload::outgoing::UpdateVoiceState;

use crate::interaction::LuroSlash;
use luro_model::database::drivers::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "leave", desc = "Leave voice in a guild", dm_permission = false)]
pub struct LeaveCommand {}

impl LuroCommand for LeaveCommand {
    async fn run_command(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let guild_id = ctx.interaction.guild_id.unwrap();
        let player = ctx.framework.lavalink.player(guild_id).await.unwrap();
        player.send(Destroy::from(guild_id))?;

        ctx.shard.command(&UpdateVoiceState::new(guild_id, None, false, false))?;
        ctx.respond(|r| r.content("Left the channel. Goodbye!")).await
    }
}
