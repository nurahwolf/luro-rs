use luro_framework::{responses::Response, CommandInteraction, ExecuteLuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Seek;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "seek", desc = "Seek through the track", dm_permission = false)]
pub struct SeekCommand {
    /// Where in the track do you want to seek to (in seconds)?
    position: i64,
}

impl ExecuteLuroCommand for SeekCommand {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let guild_id = match ctx.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.response_simple(Response::NotGuild).await,
        };

        let player = ctx.lavalink.player(guild_id).await?;
        player.send(Seek::from((guild_id, self.position * 1000)))?;

        ctx.respond(|r| r.content(format!("Seeked to {}s", self.position))).await
    }
}
