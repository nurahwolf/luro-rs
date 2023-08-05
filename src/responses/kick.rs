use anyhow::Error;
use twilight_model::{
    guild::{Guild, Member},
    user::User
};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource};

use crate::{models::SlashUser, ACCENT_COLOUR};

use crate::models::LuroSlash;

impl LuroSlash {
    pub async fn kick_response(
        mut self,
        guild: Guild,
        moderator: Member,
        banned_user: User,
        reason: &String,
        success: bool
    ) -> anyhow::Result<()> {
        let mut embed = self.kick_embed(guild, moderator, banned_user, reason).await?;
        if success {
            embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Successful").inline())
        } else {
            embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Failed").inline())
        }

        self.embed(embed.build())?.respond().await
    }

    /// Embed showing that a member got banned
    pub async fn kick_embed(
        &self,
        guild: Guild,
        moderator: Member,
        kicked_user: User,
        reason: &String
    ) -> Result<EmbedBuilder, Error> {
        let moderator = SlashUser::from_member(&moderator, Some(guild.id));
        let victim = SlashUser::from(kicked_user);

        let embed_author = EmbedAuthorBuilder::new(format!("Kicked by {} - {}", moderator.name, moderator.user_id))
            .icon_url(ImageSource::url(moderator.avatar)?)
            .build();

        let mut embed = EmbedBuilder::new()
            .color(ACCENT_COLOUR)
            .title(format!("Kicked from {}", guild.name))
            .author(embed_author)
            .field(EmbedFieldBuilder::new("Guild ID", guild.id.to_string()).inline())
            .thumbnail(ImageSource::url(victim.avatar)?);

        if !reason.is_empty() {
            embed = embed.description(format!(
                "**User:** <@{0}> - {1}\n**User ID:** {0}\n```{reason}```",
                victim.user_id, victim.name
            ))
        } else {
            embed = embed.description(format!(
                "**User:** <@{0}> - {1}\n**User ID:** {0}",
                victim.user_id, victim.name
            ))
        }

        Ok(embed)
    }
}
