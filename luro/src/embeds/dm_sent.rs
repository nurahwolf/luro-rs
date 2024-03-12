use crate::builders::EmbedBuilder;
use crate::models::emoji::{JOIN, LEAVE, MAIL};

/// Append a field to state if the response was successfully sent in a DM
pub fn dm_sent(embed: &mut EmbedBuilder, dm_success: bool) -> &mut EmbedBuilder {
    let success_text = match dm_success {
        true => format!("{MAIL}`User has been notified`{JOIN}"),
        false => format!("{MAIL}`Failed to notify user`{LEAVE}"),
    };

    match embed.0.description {
        Some(ref mut description) => description.push_str(&success_text),
        None => embed.0.description = Some(success_text),
    };

    embed
}
