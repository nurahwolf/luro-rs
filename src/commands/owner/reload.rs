use std::path::Path;

use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    models::{GuildSetting, Hecks, LuroResponse},
    LuroContext, GUILDSETTINGS_FILE_PATH, HECK_FILE_PATH
};

use crate::traits::luro_command::LuroCommand;
use crate::traits::toml::LuroTOML;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "reload",
    desc = "Reload data modified in local files. WARNING - This WILL overwrite data in memory!"
)]
pub struct ReloadCommand {}

#[async_trait]
impl LuroCommand for ReloadCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let hecks = Hecks::get(Path::new(HECK_FILE_PATH)).await?;

        {
            let global_data = &ctx.data_global;
            global_data.write().hecks = hecks
        }

        for guild_setting in &ctx.data_guild {
            let guild_settings = GuildSetting::get(Path::new(&format!(
                "{0}/{1}/guild_settings.toml",
                GUILDSETTINGS_FILE_PATH,
                guild_setting.key()
            )))
            .await?;
            ctx.data_guild.entry(*guild_setting.key()).insert_entry(guild_settings);
        }

        slash.content("Reloaded data from disk!".to_owned());
        ctx.respond(&mut slash).await
    }
}
