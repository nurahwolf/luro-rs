use anyhow::Error;
use luro_model::{database::drivers::LuroDatabaseDriver, user::LuroUser};
use twilight_model::{guild::Guild, user::User};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource};

use crate::{interaction::LuroSlash, COLOUR_DANGER};

impl<D: LuroDatabaseDriver> LuroSlash<D> {
    pub async fn ban_response(
        &self,
        guild: Guild,
        banned_user: User,
        reason: &String,
        period: &String,
        success: bool
    ) -> anyhow::Result<()> {
        let mut embed = self.ban_embed(guild, banned_user, reason, period).await?;
        if success {
            embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Successful").inline())
        } else {
            embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Failed").inline())
        }

        self.respond(|r| r.add_embed(embed.build())).await
    }

    /// An embed formatted to show a banned user
    pub async fn ban_embed(
        &self,
        guild: Guild,
        banned_user: User,
        reason: &String,
        period: &String
    ) -> Result<EmbedBuilder, Error> {
        let moderator = self.get_interaction_author(&self.interaction).await?;
        let victim = LuroUser::from(&banned_user);

        let embed_author = EmbedAuthorBuilder::new(format!("Banned by {} - {}", moderator.name(), moderator.id))
            .icon_url(ImageSource::url(moderator.avatar())?)
            .build();

        let mut embed = EmbedBuilder::new()
            .color(COLOUR_DANGER)
            .title(format!("Banned from {}", guild.name))
            .author(embed_author)
            .field(EmbedFieldBuilder::new("Purged Messages", period).inline())
            .field(EmbedFieldBuilder::new("Guild ID", guild.id.to_string()).inline())
            .thumbnail(ImageSource::url(victim.avatar())?);

        if !reason.is_empty() {
            embed = embed.description(format!(
                "**User:** <@{0}> - {1}\n**User ID:** {0}\n```{reason}```",
                victim.id, victim.name
            ))
        } else {
            embed = embed.description(format!("**User:** <@{0}> - {1}\n**User ID:** {0}", victim.id, victim.name))
        }

        Ok(embed)
    }
}
