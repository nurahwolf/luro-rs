use anyhow::Error;
use twilight_model::{
    guild::{Guild, Member},
    user::User
};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource};

use crate::{
    models::{LuroSlash, SlashUser},
    COLOUR_DANGER
};

impl LuroSlash {
    pub async fn ban_response(
        mut self,
        guild: Guild,
        moderator: Member,
        banned_user: User,
        reason: &String,
        period: &String,
        success: bool
    ) -> anyhow::Result<()> {
        let mut embed = self.ban_embed(guild, moderator, banned_user, reason, period).await?;
        if success {
            embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Successful").inline())
        } else {
            embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Failed").inline())
        }

        self.embed(embed.build())?.respond().await
    }

    /// An embed formatted to show a banned user
    pub async fn ban_embed(
        &self,
        guild: Guild,
        moderator: Member,
        banned_user: User,
        reason: &String,
        period: &String
    ) -> Result<EmbedBuilder, Error> {
        let moderator = SlashUser::from_member(&moderator.user, moderator.avatar, Some(guild.id));
        let victim = SlashUser::from(banned_user);

        let embed_author = EmbedAuthorBuilder::new(format!("Banned by {} - {}", moderator.name, moderator.user_id))
            .icon_url(ImageSource::url(moderator.avatar)?)
            .build();

        let mut embed = EmbedBuilder::new()
            .color(COLOUR_DANGER)
            .title(format!("Banned from {}", guild.name))
            .author(embed_author)
            .field(EmbedFieldBuilder::new("Purged Messages", period).inline())
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
