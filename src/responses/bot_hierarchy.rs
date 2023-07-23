use tracing::warn;
use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, models::LuroResponse, COLOUR_DANGER};

/// An embed returned if the user is above the bot in the role hierarchy.
fn bot_hierarchy_embed(bot_username: &String) -> EmbedBuilder {
    warn!("User tried to execute a command in which the bot is too low to function");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("Role Hierarchy Error")
        .description(format!("This member has a role above or equivalent to that of {bot_username} in the list of roles, which prevents moderation actions from being performed on them. You can correct this by placing me higher in the list of roles.\nHowever, if you are trying to action someone higher than me on purpose... ***Get fucked ;)***"))
}

/// A response returned if the user is above the bot in the role hierarchy.
pub fn bot_hierarchy_response(bot_username: &String, response: LuroResponse) -> InteractionResponse {
    InteractionResponse::Embed {
        embeds: vec![bot_hierarchy_embed(bot_username).build()],
        luro_response: response
    }
}
