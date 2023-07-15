use crate::interactions::InteractionResponse;

/// User is not a server member.
pub fn say(message: String, ephemeral: bool) -> InteractionResponse {
    InteractionResponse::Text {
        content: message,
        ephemeral,
    }
}
