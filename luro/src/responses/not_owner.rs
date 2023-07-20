use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, COLOUR_DANGER};

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
pub fn embed() -> EmbedBuilder {
    EmbedBuilder::new()
        .title("You are not the bot owner")
        .color(COLOUR_DANGER)
        .description("Great job motherfucker, you are not the bot owner and do not have permission to use that command. THE COMMAND IS LITERALLY NAMED OWNER ONLY! WHAT THE HECK DID YOU THINK WOULD HAPPEN!?")
}

pub fn not_owner_response() -> InteractionResponse {
    InteractionResponse::Embed {
        embeds: vec![embed().build()],
        ephemeral: true,
        deferred: true
    }
}
