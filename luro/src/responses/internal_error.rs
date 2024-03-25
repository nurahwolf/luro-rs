use luro_model::builders::EmbedBuilder;

/// A response for if the command is not known
pub fn internal_error(error: &anyhow::Error) -> EmbedBuilder {
    tracing::warn!("The error {error} was raised and handled");
    let mut embed = EmbedBuilder::default();
    embed
        .title("It's fucked")
        .colour(crate::COLOUR_DANGER)
        .description(format!("```rs\n{}```", error))
        .footer(|footer| footer.text("Okay, Houston, I believe we've had a problem here ..."));
    embed
}
