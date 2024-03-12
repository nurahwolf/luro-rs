/// A response for if the command is not known
pub fn unknown_command(name: &str) -> crate::builders::EmbedBuilder {
    tracing::warn!("The command {name} is not known, or the command is disabled");
    let mut embed = crate::builders::EmbedBuilder::default();
    embed
        .title("Unknown Command Received")
        .colour(crate::COLOUR_DANGER)
        .description(format!(
            "The command `{name}` does not exist! It might be disabled, or not yet implemented. Might be best to poke my owner about this?"
        ))
        .footer(|footer| footer.text("We had a fucky wucky!"));
    embed
}
