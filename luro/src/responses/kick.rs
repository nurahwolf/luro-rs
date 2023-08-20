use anyhow::Error;
use luro_builder::embed::EmbedBuilder;
use luro_model::{constants::ACCENT_COLOUR, luro_user::LuroUser};
use twilight_model::guild::Guild;

use crate::interaction::LuroSlash;

impl LuroSlash {
    pub async fn kick_response(
        &self,
        guild: &Guild,
        punished_user: &LuroUser,
        reason: &String,
        success: bool
    ) -> anyhow::Result<()> {
        let mut embed = self.kick_embed(guild, punished_user, reason).await?;
        if success {
            embed.create_field("DM Sent", "Successful", true);
        } else {
            embed.create_field("DM Sent", "Failed", true);
        }

        self.respond(|r| r.add_embed(embed)).await
    }

    /// Embed showing that a member got banned
    pub async fn kick_embed(&self, guild: &Guild, punished_user: &LuroUser, reason: &String) -> Result<EmbedBuilder, Error> {
        let mut embed = EmbedBuilder::default();
        let moderator = self.get_interaction_author(&self.interaction).await?;

        embed
            .colour(ACCENT_COLOUR)
            .title(format!("Banned from {}", guild.name))
            .author(|author| {
                author
                    .icon_url(punished_user.avatar())
                    .name(format!("Banned by {} - {}", moderator.name(), moderator.id))
            })
            .create_field("Guild ID", &guild.id.to_string(), true)
            .thumbnail(|thumbnail| thumbnail.url(punished_user.avatar()));

        if !reason.is_empty() {
            embed.description(format!(
                "**User:** <@{0}> - {1}\n**User ID:** {0}\n```{reason}```",
                punished_user.id, punished_user.name
            ));
        } else {
            embed.description(format!(
                "**User:** <@{0}> - {1}\n**User ID:** {0}",
                punished_user.id, punished_user.name
            ));
        }

        Ok(embed)
    }
}
