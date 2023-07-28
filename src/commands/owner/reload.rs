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
#[command(
    name = "reload",
    desc = "Reload data modified in local files. WARNING - This WILL overwrite data in memory!"
)]
pub struct ReloadCommand {}

#[async_trait]
impl LuroCommand for ReloadCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let hecks = Hecks::get(Path::new(HECK_FILE_PATH)).await?;
        let guild_settings;

        {
            let mut global_data = ctx.luro.global_data.write();
            global_data.hecks = hecks
        }

        {
            let guild_data = ctx.luro.guild_data.read();
            guild_settings = guild_data.clone()
        }

        for (guild_id, _) in guild_settings {
            let guild_settings = GuildSetting::get(Path::new(&format!(
                "{0}/{1}/guild_settings.toml",
                GUILDSETTINGS_FILE_PATH, guild_id
            )))
            .await?;
            ctx.luro.guild_data.write().entry(guild_id).insert_entry(guild_settings);
        }

        ctx.content("Reloaded data from disk!".to_owned()).respond().await
    }
}
