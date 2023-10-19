use luro_model::{builders::EmbedBuilder, COLOUR_DANGER};
use tracing::warn;

/// A response for if the command is not known
pub fn unknown_command(name: &str) -> EmbedBuilder {
    warn!("The command {name} is not known, or the command is disabled");
    let mut embed = EmbedBuilder::default();
    embed
        .title("Unknown Command Received")
        .colour(COLOUR_DANGER)
        .description(format!(
            "The command `{name}` does not exist! It might be disabled, or not yet implemented. Might be best to poke my owner about this?"
        ))
        .footer(|footer| footer.text("We had a fucky wucky!"));
    embed
}
