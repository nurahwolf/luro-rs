use async_trait::async_trait;
use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::{models::GuildSetting, responses::LuroSlash};

use super::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "info", desc = "Information on the current heck database", dm_permission = true)]
pub struct HeckInfo {}

#[async_trait]
impl LuroCommand for HeckInfo {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let mut embed = EmbedBuilder::new().title("Heck Information - Global");
        let mut global_details = String::new();
        {
            let global_data = ctx.luro.global_data.read();
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

        if let Some(guild_id) = ctx.interaction.guild_id {
            let mut guild_details = String::new();
            let guild_settings = GuildSetting::manage_guild_settings(&ctx.luro, guild_id, None, false).await?;
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

        ctx.embed(embed.build())?.respond().await
    }
}
