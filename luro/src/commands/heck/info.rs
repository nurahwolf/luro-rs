use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::Interaction,
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, LuroContext};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "info", desc = "Information on the current heck database", dm_permission = true)]
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
        let mut details = String::new();
        writeln!(details, "**TOTAL SFW HECKS:** {}", global_data.hecks.sfw_hecks.len())?;
        writeln!(details, "**TOTAL NSFW HECKS:** {}", global_data.hecks.nsfw_hecks.len())?;
        writeln!(details, "**TOTAL SFW IDS AVAILABLE:** {}", global_data.hecks.sfw_heck_ids.len())?;
        writeln!(details, "**TOTAL NSFW IDS AVAILABLE:** {}", global_data.hecks.nsfw_heck_ids.len())?;

        embed = embed.description(details);

        Ok(InteractionResponse::Embed { embeds: vec![embed.build()] , components: None, ephemeral: false })
    }
}
