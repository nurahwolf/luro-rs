use anyhow::Error;
use luro_model::{builders::EmbedBuilder, COLOUR_DANGER};
use tracing::warn;

/// A response for if the command is not known
pub fn internal_error(error: Error) -> EmbedBuilder {
    warn!("The error {error} was raised and handled");
    let mut embed = EmbedBuilder::default();
    embed
        .title("It's fucked")
        .colour(COLOUR_DANGER)
        .description(format!("```rs\n{}```", error))
        .footer(|footer| footer.text("Okay, Houston, I believe we've had a problem here ..."));
    embed
}
