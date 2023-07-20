use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Seek;
use twilight_model::application::interaction::Interaction;

use crate::{interactions::InteractionResponse, LuroContext};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "seek", desc = "Seek through the track", dm_permission = false)]
pub struct SeekCommand {
    /// Where in the track do you want to seek to (in seconds)?
    position: i64
}

impl SeekCommand {
    pub async fn run(self, interaction: &Interaction, ctx: &LuroContext) -> anyhow::Result<InteractionResponse> {
        let ephemeral = ctx.defer_interaction(interaction, true).await?;

        let guild_id = interaction.guild_id.unwrap();

        let player = ctx.lavalink.player(guild_id).await.unwrap();
        player.send(Seek::from((guild_id, self.position * 1000)))?;

        Ok(InteractionResponse::Content {
            content: format!("Seeked to {}s", self.position),
            ephemeral
        })
    }
}
