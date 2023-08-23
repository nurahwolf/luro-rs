use luro_builder::embed::EmbedBuilder;
use luro_model::{user::LuroUser, COLOUR_DANGER};
use twilight_model::id::{marker::GuildMarker, Id};

/// An embed for a standard kick action. Do not set success to a value if you do not want the DM Sent field included
pub fn user_kicked_embed(
    guild_name: &str,
    guild_id: &Id<GuildMarker>,
    punished_user: &LuroUser,
    moderator: &LuroUser,
    reason: Option<&str>,
    success: Option<bool>
) -> EmbedBuilder {
    let mut embed = EmbedBuilder::default();

    embed
        .colour(COLOUR_DANGER)
        .title(format!("👢 Kicked from {}", guild_name))
        .author(|author| {
            author
                .icon_url(moderator.avatar())
                .name(format!("Kicked by {} - {}", moderator.username(), moderator.id))
        })
        .create_field("Guild ID", &guild_id.to_string(), true)
        .thumbnail(|thumbnail| thumbnail.url(punished_user.avatar()));

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

    if let Some(success) = success {
        match success {
            true => embed.create_field("DM Sent", "Successful", true),
            false => embed.create_field("DM Sent", "Failed", true),
        };
    }

    embed
}
