use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::interaction::LuroSlash;

use crate::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "info", desc = "Information on the current heck database", dm_permission = true)]
pub struct HeckInfo {}

impl LuroCommand for HeckInfo {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let mut embed = EmbedBuilder::new().title("Heck Information - Global");
        let mut global_details = String::new();
        {
            writeln!(
                global_details,
                "**GLOBAL SFW HECKS:** {}",
                ctx.framework.database.sfw_hecks.len()
            )?;
            writeln!(
                global_details,
                "**GLOBAL NSFW HECKS:** {}",
                ctx.framework.database.nsfw_hecks.len()
            )?;
            writeln!(
                global_details,
                "**GLOBAL SFW IDS AVAILABLE:** {}",
                ctx.framework.database.available_random_sfw_hecks.read().unwrap().len()
            )?;
            writeln!(
                global_details,
                "**GLOBAL NSFW IDS AVAILABLE:** {}",
                ctx.framework.database.available_random_nsfw_hecks.read().unwrap().len()
            )?;
        }

        embed = embed.field(EmbedFieldBuilder::new("Global Stats", global_details).inline());

        if let Some(guild_id) = ctx.interaction.guild_id {
            let mut guild_details = String::new();
            let guild_settings = ctx.framework.database.get_guild(&guild_id).await?;
            writeln!(guild_details, "**GUILD SFW HECKS:** {}", guild_settings.sfw_hecks.len())?;
            writeln!(guild_details, "**GUILD NSFW HECKS:** {}", guild_settings.nsfw_hecks.len())?;
            // writeln!(
            //     guild_details,
            //     "**GUILD SFW IDS AVAILABLE:** {}",
            //     guild_settings.sfw_heck_ids.len()
            // )?;
            // writeln!(
            //     guild_details,
            //     "**GUILD NSFW IDS AVAILABLE:** {}",
            //     guild_settings.nsfw_heck_ids.len()
            // )?;
            embed = embed.field(EmbedFieldBuilder::new("Guild Stats", guild_details).inline());
        }

        ctx.respond(|r| r.add_embed(embed.build())).await
    }
}
