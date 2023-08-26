use luro_builder::embed::EmbedBuilder;
use luro_model::COLOUR_DANGER;
use tracing::warn;

/// A response for if the command is not known
pub fn unknown_command(name: &str) -> EmbedBuilder {
    warn!("The command {name} is not known");
    let mut embed = EmbedBuilder::default();
    embed
        .title("Unknown Command Received")
        .colour(COLOUR_DANGER)
        .description(format!(
            "The command `{name}` does not yet exist! Really sorry about this! Blame my owner..."
        ))
        .footer(|footer| footer.text("We had a fucky wucky!"));
    embed
}
