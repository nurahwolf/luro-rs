use luro_framework::{responses::Response, CommandInteraction, ExecuteLuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Volume;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "volume", desc = "Set the volume of the player!", dm_permission = false)]
pub struct VolumeCommand {
    /// Sets the volume between 0 and 1000! 100 is the default (100% volume)
    #[command(min_value = 0, max_value = 1000)]
    volume: i64,
}

impl ExecuteLuroCommand for VolumeCommand {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let guild_id = match ctx.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.response_simple(Response::NotGuild).await,
        };

        let player = ctx.lavalink.player(guild_id).await?;
        player.send(Volume::from((guild_id, self.volume)))?;

        ctx.respond(|r| r.content(format!("Set the volume to {}", self.volume))).await
    }
}
