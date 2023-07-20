use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;

use crate::{functions::interaction_context, guild::LuroGuilds, hecks::Hecks, interactions::InteractionResponse, LuroContext};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "save", desc = "Flush data to disk")]
pub struct SaveCommand {}

impl SaveCommand {
    pub async fn run(self, interaction: &Interaction, ctx: &LuroContext) -> anyhow::Result<InteractionResponse> {
        let ephemeral = ctx.defer_interaction(interaction, true).await?;
        let (_, _, _) = interaction_context(interaction, "heck add")?;
        Hecks::write(ctx).await?;
        LuroGuilds::write(ctx).await?;

        // Config::write(&ctx.data().config.write().await.clone(), CONFIG_FILE_PATH).await;
        // Hecks::write(data.clone().global_data.read().hecks.clone(), HECK_FILE_PATH).await?;
        // Quotes::write(&ctx.data().quotes.write().await.clone(), QUOTES_FILE_PATH).await;
        // Stories::write(&ctx.data().stories.write().await.clone(), STORIES_FILE_PATH).await;

        Ok(InteractionResponse::Content {
            content: "Flushed data to disk!".to_string(),
            ephemeral
        })
    }
}
