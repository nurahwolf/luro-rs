use crate::types::{PunishmentType, User, Guild};

pub fn new_punishment_embed(
    guild: &Guild,
    kind: PunishmentType,
    moderator: &User,
    target: &User,
) -> crate::builders::EmbedBuilder {
    let mut embed = crate::builders::EmbedBuilder::default();

    embed
        .create_field("Guild ID", &guild.guild_id.to_string(), true)
        .thumbnail(|thumbnail| thumbnail.url(target.avatar_url()));

    match kind {
        PunishmentType::Kicked(punishment_reason) => {
            reason(&mut embed, punishment_reason.as_deref(), target);
            embed
                .title(format!("👢 Kicked from {}", guild.name))
                .colour(crate::COLOUR_DANGER)
                .author(|author| {
                    author
                        .icon_url(moderator.avatar_url())
                        .name(format!("Kicked by {} - {}", moderator.username(), moderator.user_id))
                });
        }
        PunishmentType::Banned(punishment_reason, purged_message_seconds) => {
            reason(&mut embed, punishment_reason.as_deref(), target);
            purged_messages(&mut embed, purged_message_seconds);
            embed
                .title(format!("🔨 Banned from {}", guild.name))
                .colour(crate::COLOUR_DANGER)
                .author(|author| {
                    author
                        .icon_url(moderator.avatar_url())
                        .name(format!("Banned by {} - {}", moderator.username(), moderator.user_id))
                });
        }
        PunishmentType::Unbanned(punishment_reason) => {
            reason(&mut embed, punishment_reason.as_deref(), target);
            embed
                .title(format!("🔓 Unbanned from {}", guild.name))
                .colour(crate::COLOUR_SUCCESS)
                .author(|author| {
                    author
                        .icon_url(moderator.avatar_url())
                        .name(format!("Unbanned by {} - {}", moderator.username(), moderator.user_id))
                });
        }
    };
    embed
}


/// Append a period of purged messages
pub fn purged_messages(embed: &mut crate::builders::EmbedBuilder, purged_message_seconds: i64) -> &mut crate::builders::EmbedBuilder {
    let period = match purged_message_seconds {
        0 => "No messages deleted".to_owned(),
        3_600 => "Previous Hour".to_owned(),
        21_600 => "Previous 6 Hours".to_owned(),
        43_200 => "Previous 12 Hours".to_owned(),
        86_400 => "Previous 24 Hours".to_owned(),
        259_200 => "Previous 3 Days".to_owned(),
        604_800 =>  "Previous 7 Days".to_owned(),
        num => format!("Deleted `{num}` seconds worth of messages")
    };

    embed.create_field("Purged Messages", &period, true);
    embed
}

/// Append a reason to why they were actioned
fn reason<'a>(
    embed: &'a mut crate::builders::EmbedBuilder,
    reason: Option<&str>,
    punished_user: &User,
) -> &'a mut crate::builders::EmbedBuilder {
    match reason {
        Some(reason) => {
            if reason.starts_with("```") {
                embed.description(format!(
                    "**User:** <@{0}> - {1}\n**User ID:** {0}\n{reason}",
                    punished_user.user_id, punished_user.name
                ))
            } else {
                embed.description(format!(
                    "**User:** <@{0}> - {1}\n**User ID:** {0}\n```{reason}```",
                    punished_user.user_id, punished_user.name
                ))
            }
        }
        None => embed.description(format!(
            "**User:** <@{0}> - {1}\n**User ID:** {0}",
            punished_user.user_id, punished_user.name
        )),
    };
    embed
}
