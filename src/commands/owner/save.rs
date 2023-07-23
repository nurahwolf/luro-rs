use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;

use crate::{
    interactions::InteractionResponse,
    models::{GuildSettings, Hecks},
    LuroContext, SlashResponse
};

use super::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "save", desc = "Flush data to disk")]
pub struct SaveCommand {}

#[async_trait]
impl LuroCommand for SaveCommand {
    async fn run_command(self, interaction: Interaction, ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
        let luro_response = ctx.defer_interaction(&interaction, true).await?;
        let (_, _, _) = self.interaction_context(&interaction, "heck add")?;
        Hecks::write(&ctx).await?;
        GuildSettings::write(&ctx).await?;

        // Config::write(&ctx.data().config.write().await.clone(), CONFIG_FILE_PATH).await;
        // Hecks::write(data.clone().global_data.read().hecks.clone(), HECK_FILE_PATH).await?;
        // Quotes::write(&ctx.data().quotes.write().await.clone(), QUOTES_FILE_PATH).await;
        // Stories::write(&ctx.data().stories.write().await.clone(), STORIES_FILE_PATH).await;

        Ok(InteractionResponse::Content {
            content: "Flushed data to disk!".to_string(),
            luro_response
        })
    }
}
