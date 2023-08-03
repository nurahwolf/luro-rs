use anyhow::Error;
use twilight_model::{
    guild::{Guild, Member},
    user::User
};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource};

use crate::{traits::luro_functions::LuroFunctions, ACCENT_COLOUR};

use crate::models::LuroSlash;

impl LuroSlash {
    pub async fn kick_response(
        self,
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
        let (moderator, moderator_avatar, moderator_name) = self.get_member(&moderator, guild.id).await?;
        let (kicked_user, kicked_avatar, kicked_name) = self.fetch_specified_user(&self.luro, &kicked_user.id).await?;

        let embed_author = EmbedAuthorBuilder::new(format!("Kicked by {} - {}", moderator_name, moderator.user.id))
            .icon_url(ImageSource::url(moderator_avatar)?)
            .build();

        let mut embed = EmbedBuilder::new()
            .color(ACCENT_COLOUR)
            .title(format!("Kicked from {}", guild.name))
            .author(embed_author)
            .field(EmbedFieldBuilder::new("Guild ID", guild.id.to_string()).inline())
            .thumbnail(ImageSource::url(kicked_avatar)?);

        if !reason.is_empty() {
            embed = embed.description(format!(
                "**User:** <@{0}> - {kicked_name}\n**User ID:** {0}\n```{reason}```",
                kicked_user.id
            ))
        } else {
            embed = embed.description(format!("**User:** <@{0}> - {kicked_name}\n**User ID:** {0}", kicked_user.id))
        }

        Ok(embed)
    }
}
