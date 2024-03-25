use crate::models::emoji::*;

/// An embed sent to a user when they are banned
pub fn ban_user(data: &super::BannedResponse<'_>, guild_name: &str) -> crate::builders::EmbedBuilder {
    let mut embed = crate::builders::EmbedBuilder::default();

    let reason = reason_formatter(data.reason);
    let target_name = data.target.username();
    let target_id = data.target.user_id;
    let purged_messages = match data.purged_messages {
        0 => "No messages deleted".to_owned(),
        3_600 => "Previous Hour".to_owned(),
        21_600 => "Previous 6 Hours".to_owned(),
        43_200 => "Previous 12 Hours".to_owned(),
        86_400 => "Previous 24 Hours".to_owned(),
        259_200 => "Previous 3 Days".to_owned(),
        604_800 => "Previous 7 Days".to_owned(),
        num => format!("Deleted {num} seconds worth of messages"),
    };

    let target_id_usize = usize::try_from(target_id.get()).unwrap_or_default();
    let longest_word_length = target_name
        .len()
        .max(target_id_usize)
        .max(purged_messages.len().max(guild_name.len()));

    let mut description =
        format!("<@{target_id}>\n<:member:1175114506465198171>`{target_name:^longest_word_length$}` `{target_id:^longest_word_length$}`");

    description.push_str(&format!("<:private:1175114613172473987>`{purged_messages:^longest_word_length$}` `{guild_name:^longest_word_length$}`<:guide:1175114529701625977>"));
    description.push_str(&format!("<:ticket:1175114633506455704>{reason}"));

    embed
        .colour(crate::COLOUR_DANGER)
        .thumbnail(|thumbnail| thumbnail.url(data.target.avatar_url()))
        .description(description)
        .footer(|footer| footer.text(format!("Guild: {guild_name}")))
        .author(|author| {
            author
                .icon_url(data.moderator.avatar_url())
                .name(format!("BANNED by {}!", data.moderator.username()))
        });
    embed
}

/// An embed sent in response to a ban command, or the log channel
pub fn ban_logged(data: &BannedResponse<'_>, dm_success: &bool) -> EmbedBuilder {
    const DM_SUCCESS: &str = "DM Successful";
    const DM_FAILED: &str = "DM Failure";
    let mut embed = EmbedBuilder::default();

    let reason = reason_formatter(data.reason);
    let target_name = data.target.username();
    let target_id = data.target.user_id;

    let mut description = format!("<@{target_id}>\n{MEMBER}`{target_name:^longest_word_length$}` `{target_id:^longest_word_length$}`\n");

    embed
        .colour(crate::COLOUR_DANGER)
        .description(description)
        .thumbnail(|thumbnail| thumbnail.url(data.target.avatar_url()))
        .author(|author| {
            author
                .icon_url(data.moderator.avatar_url())
                .name(format!("BANNED by {}!", data.moderator.username()))
        });
    embed
}

/// Format a reason, handling if there is a code block and escaped characters
fn reason_formatter(reason: Option<&str>) -> String {
    match reason {
        Some(reason) => {
            if reason.contains('`') || reason.starts_with("```") {
                return format!("\n{reason}");
            }

            format!("`{reason}`")
        }
        None => "`No reason specified.`".to_owned(),
    }
}
