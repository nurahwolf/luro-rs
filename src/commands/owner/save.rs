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
#[command(name = "save", desc = "Flush data to disk")]
pub struct SaveCommand {}

#[async_trait]
impl LuroCommand for SaveCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let hecks;
        {
            let global_data = ctx.data_global.read();
            hecks = global_data.hecks.clone()
        }

        Hecks::write(&hecks, Path::new(HECK_FILE_PATH)).await?;

        for guild_setting in &ctx.data_guild {
            GuildSetting::write(
                guild_setting.value(),
                Path::new(&format!(
                    "{0}/{1}/guild_settings.toml",
                    GUILDSETTINGS_FILE_PATH,
                    guild_setting.key()
                ))
            )
            .await?;
        }

        slash.content("Flushed data to disk!".to_owned());
        ctx.respond(&mut slash).await
    }
}
