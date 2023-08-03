use anyhow::Error;
use twilight_model::{
    guild::{Guild, Member},
    user::User
};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource};

use crate::{models::LuroSlash, traits::luro_functions::LuroFunctions, ACCENT_COLOUR};

impl LuroSlash {
    pub async fn ban_response(
        self,
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
        let (moderator, moderator_avatar, moderator_name) = self.get_member(&moderator, guild.id).await?;
        let (banned, banned_avatar, banned_name) = self.fetch_specified_user(&self.luro, &banned_user.id).await?;

        let embed_author = EmbedAuthorBuilder::new(format!("Banned by {} - {}", moderator_name, moderator.user.id))
            .icon_url(ImageSource::url(moderator_avatar)?)
            .build();

        let mut embed = EmbedBuilder::new()
            .color(ACCENT_COLOUR)
            .title(format!("Banned from {}", guild.name))
            .author(embed_author)
            .field(EmbedFieldBuilder::new("Purged Messages", period).inline())
            .field(EmbedFieldBuilder::new("Guild ID", guild.id.to_string()).inline())
            .thumbnail(ImageSource::url(banned_avatar)?);

        if !reason.is_empty() {
            embed = embed.description(format!(
                "**User:** <@{0}> - {banned_name}\n**User ID:** {0}\n```{reason}```",
                banned.id
            ))
        } else {
            embed = embed.description(format!("**User:** <@{0}> - {banned_name}\n**User ID:** {0}", banned.id))
        }

        Ok(embed)
    }
}
