use std::path::Path;

use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    models::{GuildSetting, Hecks},
    responses::LuroSlash,
    GUILDSETTINGS_FILE_PATH, HECK_FILE_PATH
};

use super::LuroCommand;
use crate::models::toml::LuroTOML;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "save", desc = "Flush data to disk")]
pub struct SaveCommand {}

#[async_trait]
impl LuroCommand for SaveCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let hecks;
        let guild_settings;
        {
            let global_data = ctx.luro.global_data.read();
            hecks = global_data.hecks.clone()
        }

        {
            let guild_data = ctx.luro.guild_data.read();
            guild_settings = guild_data.clone()
        }

        Hecks::write(&hecks, Path::new(HECK_FILE_PATH)).await?;

        for (guild_id, guild_settings) in guild_settings {
            GuildSetting::write(
                &guild_settings,
                Path::new(&format!("{0}/{1}/{1}.toml", GUILDSETTINGS_FILE_PATH, guild_id))
            )
            .await?;
        }

        ctx.content("Flushed data to disk!".to_owned()).respond().await
    }
}
