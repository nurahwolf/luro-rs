use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_lavalink::model::Pause;
use twilight_model::application::interaction::Interaction;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{interactions::InteractionResponse, LuroContext};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "pause", desc = "Pause / Unpause the current playing music", dm_permission = false)]
pub struct PauseCommand {}

impl PauseCommand {
    pub async fn run(self, interaction: &Interaction, ctx: &LuroContext) -> anyhow::Result<InteractionResponse> {
        let ephemeral = ctx.defer_interaction(interaction, true).await?;

        let guild_id = interaction.guild_id.unwrap();

        let player = ctx.lavalink.player(guild_id).await.unwrap();
        let paused = player.paused();
        player.send(Pause::from((guild_id, !paused)))?;

        let action = if paused { "Unpaused " } else { "Paused" };
        let _response = InteractionResponseDataBuilder::new().content(format!("{action} the track"));

        Ok(InteractionResponse::Content {
            content: format!("{action} the track"),
            ephemeral,
            deferred: true
        })
    }
}
