use anyhow::Error;
use luro_model::{constants::ACCENT_COLOUR, luro_user::LuroUser};
use twilight_model::{guild::Guild, user::User};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource};

use crate::interaction::LuroSlash;

impl LuroSlash {
    pub async fn kick_response(&self, guild: Guild, banned_user: User, reason: &String, success: bool) -> anyhow::Result<()> {
        let mut embed = self.kick_embed(guild, banned_user, reason).await?;
        if success {
            embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Successful").inline())
        } else {
            embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Failed").inline())
        }

        self.respond(|r| r.add_embed(embed.build())).await
    }

    /// Embed showing that a member got banned
    pub async fn kick_embed(&self, guild: Guild, kicked_user: User, reason: &String) -> Result<EmbedBuilder, Error> {
        let moderator = self.get_interaction_author(&self.interaction).await?;
        let victim = LuroUser::from(&kicked_user);

        let embed_author = EmbedAuthorBuilder::new(format!("Kicked by {} - {}", moderator.name, moderator.id))
            .icon_url(ImageSource::url(moderator.avatar())?)
            .build();

        let mut embed = EmbedBuilder::new()
            .color(ACCENT_COLOUR)
            .title(format!("Kicked from {}", guild.name))
            .author(embed_author)
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
