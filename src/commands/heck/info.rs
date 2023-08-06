use async_trait::async_trait;
use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::models::{GuildSetting, LuroResponse};
use crate::LuroContext;

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "info", desc = "Information on the current heck database", dm_permission = true)]
pub struct HeckInfo {}

#[async_trait]
impl LuroCommand for HeckInfo {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let mut embed = EmbedBuilder::new().title("Heck Information - Global");
        let mut global_details = String::new();
        {
            let global_data = ctx.data_global.read();
            writeln!(global_details, "**GLOBAL SFW HECKS:** {}", global_data.hecks.sfw_hecks.len())?;
            writeln!(
                global_details,
                "**GLOBAL NSFW HECKS:** {}",
                global_data.hecks.nsfw_hecks.len()
            )?;
            writeln!(
                global_details,
                "**GLOBAL SFW IDS AVAILABLE:** {}",
                global_data.hecks.sfw_heck_ids.len()
            )?;
            writeln!(
                global_details,
                "**GLOBAL NSFW IDS AVAILABLE:** {}",
                global_data.hecks.nsfw_heck_ids.len()
            )?;
        }

        embed = embed.field(EmbedFieldBuilder::new("Global Stats", global_details).inline());

        if let Some(guild_id) = slash.interaction.guild_id {
            let mut guild_details = String::new();
            let guild_settings = GuildSetting::get_guild_settings(ctx, &guild_id).await?;
            writeln!(guild_details, "**GUILD SFW HECKS:** {}", guild_settings.hecks.sfw_hecks.len())?;
            writeln!(
                guild_details,
                "**GUILD NSFW HECKS:** {}",
                guild_settings.hecks.nsfw_hecks.len()
            )?;
            writeln!(
                guild_details,
                "**GUILD SFW IDS AVAILABLE:** {}",
                guild_settings.hecks.sfw_heck_ids.len()
            )?;
            writeln!(
                guild_details,
                "**GUILD NSFW IDS AVAILABLE:** {}",
                guild_settings.hecks.nsfw_heck_ids.len()
            )?;
            embed = embed.field(EmbedFieldBuilder::new("Guild Stats", guild_details).inline());
        }

        slash.embed(embed.build())?;
        ctx.respond(&mut slash).await
    }
}
