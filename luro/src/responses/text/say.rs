use twilight_model::channel::message::Component;

use crate::interactions::InteractionResponse;

/// User is not a server member.
pub fn say(
    message: String,
    components: Option<Vec<Component>>,
    ephemeral: bool,
) -> InteractionResponse {
    InteractionResponse::Text {
        content: message,
        components,
        ephemeral,
    }
}
