use async_trait::async_trait;
use std::fmt::Write;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::{interactions::InteractionResponse, LuroContext, SlashResponse};

use super::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "info", desc = "Information on the current heck database", dm_permission = true)]
pub struct HeckInfo {}

#[async_trait]
impl LuroCommand for HeckInfo {
    async fn run_command(self, interaction: Interaction, ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
        let ephemeral = ctx.defer_interaction(&interaction, true).await?;
        let (_, _, _) = self.interaction_context(&interaction, "'heck info' command invoked")?;
        let global_data = ctx.global_data.read();

        let mut embed = EmbedBuilder::new().title("Heck Information - Global");
        let mut global_details = String::new();
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
        embed = embed.field(EmbedFieldBuilder::new("Global Stats", global_details).inline());

        if let Some(guild_id) = interaction.guild_id && let Some(guild_settings) = ctx.guild_data.read().get(&guild_id) {
            let mut guild_details = String::new();
            writeln!(guild_details, "**GUILD SFW HECKS:** {}", guild_settings.hecks.sfw_hecks.len())?;
            writeln!(guild_details, "**GUILD NSFW HECKS:** {}", guild_settings.hecks.nsfw_hecks.len())?;
            writeln!(guild_details, "**GUILD SFW IDS AVAILABLE:** {}", guild_settings.hecks.sfw_heck_ids.len())?;
            writeln!(guild_details, "**GUILD NSFW IDS AVAILABLE:** {}", guild_settings.hecks.nsfw_heck_ids.len())?;
            embed = embed.field(EmbedFieldBuilder::new("Guild Stats", guild_details).inline());
        }

        Ok(InteractionResponse::Embed {
            embeds: vec![embed.build()],
            ephemeral,
            deferred: true
        })
    }
}
