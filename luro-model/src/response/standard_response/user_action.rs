use crate::types::{Guild, PunishmentType, User};
use std::fmt::Write;

pub fn new_punishment_embed(
    guild: &Guild,
    kind: &PunishmentType,
    moderator: &User,
    target: &User,
) -> anyhow::Result<crate::builders::EmbedBuilder> {
    let mut description = String::new();
    let mut embed = crate::builders::EmbedBuilder::default();
    embed.thumbnail(|thumbnail| thumbnail.url(target.avatar_url()));

    match kind {
        PunishmentType::Kicked(punishment_reason) => {
            writeln!(
                description,
                "<:member:1175114506465198171> <@{1}> - {} `{1}`",
                target.username(),
                target.user_id
            )?;
            writeln!(
                description,
                "<:guide:1175114529701625977> **Guild:** {} `{}`",
                guild.name, guild.guild_id
            )?;
            writeln!(
                description,
                "<:ticket:1175114633506455704> **Reason:** {}",
                reason_formatter(punishment_reason.as_deref())
            )?;

            embed.colour(crate::COLOUR_DANGER).description(description).author(|author| {
                author
                    .icon_url(moderator.avatar_url())
                    .name(format!("KICKED by {}!", moderator.username()))
            });
        }
        PunishmentType::Banned(punishment_reason, purged_message_seconds) => {
            writeln!(
                description,
                "<:member:1175114506465198171> <@{1}> - {} `{1}`",
                target.username(),
                target.user_id
            )?;
            writeln!(
                description,
                "<:guide:1175114529701625977> **Guild:** {} `{}`",
                guild.name, guild.guild_id
            )?;
            writeln!(
                description,
                "<:private:1175114613172473987> **Messages Deleted:** {}",
                match purged_message_seconds {
                    0 => "`No messages deleted`".to_owned(),
                    3_600 => "`Previous Hour`".to_owned(),
                    21_600 => "`Previous 6 Hours`".to_owned(),
                    43_200 => "`Previous 12 Hours`".to_owned(),
                    86_400 => "`Previous 24 Hours`".to_owned(),
                    259_200 => "`Previous 3 Days`".to_owned(),
                    604_800 => "`Previous 7 Days`".to_owned(),
                    num => format!("Deleted `{num}` seconds worth of messages"),
                }
            )?;
            writeln!(
                description,
                "<:ticket:1175114633506455704> **Reason:** {}",
                reason_formatter(*punishment_reason)
            )?;

            embed.colour(crate::COLOUR_DANGER).description(description).author(|author| {
                author
                    .icon_url(moderator.avatar_url())
                    .name(format!("BANNED by {}!", moderator.username()))
            });
        }
        PunishmentType::Unbanned(punishment_reason) => {
            writeln!(
                description,
                "<:member:1175114506465198171> <@{1}> - {} `{1}`",
                target.username(),
                target.user_id
            )?;
            writeln!(
                description,
                "<:guide:1175114529701625977> **Guild:** {} `{}`",
                guild.name, guild.guild_id
            )?;
            writeln!(
                description,
                "<:ticket:1175114633506455704> **Reason:** {}",
                reason_formatter(punishment_reason.as_deref())
            )?;

            embed.colour(crate::COLOUR_SUCCESS).description(description).author(|author| {
                author
                    .icon_url(moderator.avatar_url())
                    .name(format!("UNBANNED by {}!", moderator.username()))
            });
        }
    };

    Ok(embed)
}

/// Format a reason, handling if there is a code block and escaped characters
fn reason_formatter(reason: Option<&str>) -> String {
    match reason {
        Some(reason) => {
            if reason.contains('`') || reason.starts_with("```") {
                return reason.to_owned();
            }

            format!("`{reason}`")
        }
        None => "`No reason specified.`".to_owned(),
    }
}
