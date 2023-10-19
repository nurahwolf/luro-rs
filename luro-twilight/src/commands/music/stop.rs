use luro_framework::{ExecuteLuroCommand, CommandInteraction, responses::Response};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Stop;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "stop", desc = "Stop the currently playing track", dm_permission = false)]
pub struct StopCommand {}

impl ExecuteLuroCommand for StopCommand {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let guild_id = match ctx.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.response_simple(Response::NotGuild).await,
        };

        let player = ctx.lavalink.player(guild_id).await?;
        player.send(Stop::from(guild_id))?;

        ctx.respond(|r| r.content("Stopped the track!")).await
    }
}
