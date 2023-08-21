use luro_builder::embed::EmbedBuilder;
use luro_model::{database::drivers::LuroDatabaseDriver, user::LuroUser};
use twilight_model::{
    guild::Guild,
    id::{marker::GuildMarker, Id}
};

use crate::{framework::Framework, interaction::LuroSlash, COLOUR_DANGER};

impl<D: LuroDatabaseDriver> LuroSlash<D> {
    pub async fn ban_response(
        &self,
        guild: &Guild,
        punished_user: &LuroUser,
        reason: Option<&str>,
        period: &String,
        success: bool
    ) -> anyhow::Result<()> {
        let moderator = self.get_interaction_author(&self.interaction).await?;
        let mut embed = self
            .framework
            .ban_embed(&guild.name, &guild.id, &moderator, punished_user, reason, Some(period));
        if success {
            embed.create_field("DM Sent", "Successful", true);
        } else {
            embed.create_field("DM Sent", "Failed", true);
        }

        self.respond(|r| r.add_embed(embed)).await
    }
}

impl<D: LuroDatabaseDriver> Framework<D> {
    /// An embed formatted to show a banned user
    pub fn ban_embed(
        &self,
        guild_name: &str,
        guild_id: &Id<GuildMarker>,
        moderator: &LuroUser,
        punished_user: &LuroUser,
        reason: Option<&str>,
        period: Option<&str>
    ) -> EmbedBuilder {
        let mut embed = EmbedBuilder::default();

        embed
            .colour(COLOUR_DANGER)
            .title(format!("🔨 Banned from {}", guild_name))
            .author(|author| {
                author
                    .icon_url(moderator.avatar())
                    .name(format!("Banned by {} - {}", moderator.username(), moderator.id))
            })
            .create_field("Guild ID", &guild_id.to_string(), true)
            .thumbnail(|thumbnail| thumbnail.url(punished_user.avatar()));

        if let Some(period) = period {
            embed.create_field("Purged Messages", period, true);
        }

        match reason {
            Some(reason) => {
                if reason.starts_with("```") {
                    embed.description(format!(
                        "**User:** <@{0}> - {1}\n**User ID:** {0}\n{reason}",
                        punished_user.id, punished_user.name
                    ))
                } else {
                    embed.description(format!(
                        "**User:** <@{0}> - {1}\n**User ID:** {0}\n```{reason}```",
                        punished_user.id, punished_user.name
                    ))
                }
            }
            None => embed.description(format!(
                "**User:** <@{0}> - {1}\n**User ID:** {0}",
                punished_user.id, punished_user.name
            ))
        };

        embed
    }
}
