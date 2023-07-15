use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::{interactions::InteractionResponse, LuroContext};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "info",
    desc = "Information on the current heck database",
    dm_permission = true
)]
pub struct HeckInfo {}

impl HeckInfo {
    pub async fn run(
        self,
        ctx: LuroContext,
        interaction: &Interaction,
    ) -> anyhow::Result<InteractionResponse> {
        tracing::debug!(
            "heck user command in channel {} by {}",
            interaction.channel.clone().unwrap().name.unwrap(),
            interaction.user.clone().unwrap().name
        );
        let global_data = ctx.global_data.read();

        let mut embed = EmbedBuilder::new().title("Heck Information - Global");
        let mut global_details = String::new();
        writeln!(
            global_details,
            "**GLOBAL SFW HECKS:** {}",
            global_data.hecks.sfw_hecks.len()
        )?;
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
        embed = embed.field(EmbedFieldBuilder::new("Global Stats", global_details).inline());

        if let Some(guild_id) = interaction.guild_id && let Some(guild_settings) = ctx.guilds.read().get(&guild_id) {
            let mut guild_details = String::new();
            writeln!(guild_details, "**GUILD SFW HECKS:** {}", guild_settings.hecks.sfw_hecks.len())?;
            writeln!(guild_details, "**GUILD NSFW HECKS:** {}", guild_settings.hecks.nsfw_hecks.len())?;
            writeln!(guild_details, "**GUILD SFW IDS AVAILABLE:** {}", guild_settings.hecks.sfw_heck_ids.len())?;
            writeln!(guild_details, "**GUILD NSFW IDS AVAILABLE:** {}", guild_settings.hecks.nsfw_heck_ids.len())?;
            embed = embed.field(EmbedFieldBuilder::new("Guild Stats", guild_details).inline());
        }

        Ok(InteractionResponse::Embed {
            embeds: vec![embed.build()],
            components: None,
            ephemeral: false,
        })
    }
}
