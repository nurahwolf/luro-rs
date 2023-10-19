use luro_framework::{ExecuteLuroCommand, CommandInteraction, responses::Response};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Pause;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "pause", desc = "Pause / Unpause the current playing music", dm_permission = false)]
pub struct PauseCommand {}

impl ExecuteLuroCommand for PauseCommand {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let guild_id = match ctx.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.response_simple(Response::NotGuild).await,
        };

        let player = ctx.lavalink.player(guild_id).await?;
        let paused = player.paused();
        player.send(Pause::from((guild_id, !paused)))?;

        let actioned = if paused { "Unpaused " } else { "Paused" };
        ctx.respond(|r| r.content(format!("{actioned} the track!"))).await
    }
}
