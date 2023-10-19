use luro_framework::{responses::Response, CommandInteraction, ExecuteLuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Destroy;
use twilight_model::gateway::payload::outgoing::UpdateVoiceState;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "leave", desc = "Leave voice in a guild", dm_permission = false)]
pub struct LeaveCommand {}

impl ExecuteLuroCommand for LeaveCommand {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let guild_id = match ctx.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.response_simple(Response::NotGuild).await,
        };
        let player = ctx.lavalink.player(guild_id).await?;
        player.send(Destroy::from(guild_id))?;

        ctx.shard.command(&UpdateVoiceState::new(guild_id, None, false, false))?;
        ctx.respond(|r| r.content("Left the channel. Goodbye!")).await
    }
}
