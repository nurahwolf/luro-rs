use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, COLOUR_DANGER};

/// User is above Luro in the role hierarchy.
pub fn bot_hierarchy(bot_username: &String) -> InteractionResponse {
    let embed = EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("Role Hierarchy Error")
        .description(format!("This member has a role above or equivalent to that of {bot_username} in the list of roles, which prevents moderation actions from being performed on them. You can correct this by placing me higher in the list of roles.\nHowever, if you are trying to action someone higher than me on purpose... ***Get fucked ;)***"))
        .build();

    InteractionResponse::Embed {
        embeds: vec![embed],
        ephemeral: true
    }
}
