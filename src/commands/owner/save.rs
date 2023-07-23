use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    models::{GuildSettings, Hecks},
    responses::LuroSlash
};

use super::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "save", desc = "Flush data to disk")]
pub struct SaveCommand {}

#[async_trait]
impl LuroCommand for SaveCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        Hecks::write(&ctx.luro).await?;
        GuildSettings::write(&ctx.luro).await?;

        // Config::write(&ctx.data().config.write().await.clone(), CONFIG_FILE_PATH).await;
        // Hecks::write(data.clone().global_data.read().hecks.clone(), HECK_FILE_PATH).await?;
        // Quotes::write(&ctx.data().quotes.write().await.clone(), QUOTES_FILE_PATH).await;
        // Stories::write(&ctx.data().stories.write().await.clone(), STORIES_FILE_PATH).await;
        ctx.content("Flushed data to disk!".to_owned()).respond().await
    }
}
