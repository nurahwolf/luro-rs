use luro_model::{user::LuroUser, COLOUR_DANGER, COLOUR_SUCCESS};
use twilight_model::id::{marker::GuildMarker, Id};

use super::{PunishmentType, StandardResponse};

impl StandardResponse {
    /// Create a new punishment response
    pub fn new_punishment(
        kind: PunishmentType,
        guild_name: &str,
        guild_id: &Id<GuildMarker>,
        punished_user: &LuroUser,
        moderator: &LuroUser
    ) -> Self {
        let mut response = StandardResponse::default();
        response
            .embed
            .create_field("Guild ID", &guild_id.to_string(), true)
            .thumbnail(|thumbnail| thumbnail.url(punished_user.avatar()));
        match kind {
            PunishmentType::Kicked => {
                response
                    .embed
                    .title(format!("👢 Kicked from {}", guild_name))
                    .colour(COLOUR_DANGER)
                    .author(|author| {
                        author.icon_url(moderator.avatar()).name(format!(
                            "Kicked by {} - {}",
                            moderator.username(),
                            moderator.id
                        ))
                    });
            }
            PunishmentType::Banned => {
                response
                    .embed
                    .title(format!("🔨 Banned from {}", guild_name))
                    .colour(COLOUR_DANGER)
                    .author(|author| {
                        author.icon_url(moderator.avatar()).name(format!(
                            "Banned by {} - {}",
                            moderator.username(),
                            moderator.id
                        ))
                    });
            }
            PunishmentType::Unbanned => {
                response
                    .embed
                    .title(format!("🔓 Unbanned from {}", guild_name))
                    .colour(COLOUR_SUCCESS)
                    .author(|author| {
                        author.icon_url(moderator.avatar()).name(format!(
                            "Unbanned by {} - {}",
                            moderator.username(),
                            moderator.id
                        ))
                    });
            }
        };
        response
    }

    /// Append a period of purged messages
    pub fn punishment_period(&mut self, period: &str) -> &mut Self {
        self.embed.create_field("Purged Messages", period, true);
        self
    }

    /// Append a reason to why they were actioned
    pub fn punishment_reason(&mut self, reason: Option<&str>, punished_user: &LuroUser) -> &mut Self {
        match reason {
            Some(reason) => {
                if reason.starts_with("```") {
                    self.embed.description(format!(
                        "**User:** <@{0}> - {1}\n**User ID:** {0}\n{reason}",
                        punished_user.id, punished_user.name
                    ))
                } else {
                    self.embed.description(format!(
                        "**User:** <@{0}> - {1}\n**User ID:** {0}\n```{reason}```",
                        punished_user.id, punished_user.name
                    ))
                }
            }
            None => self.embed.description(format!(
                "**User:** <@{0}> - {1}\n**User ID:** {0}",
                punished_user.id, punished_user.name
            ))
        };
        self
    }
}
