/// A response for if the command is not known
pub fn internal_error(error: anyhow::Error) -> crate::builders::EmbedBuilder {
    tracing::warn!("The error {error} was raised and handled");
    let mut embed = crate::builders::EmbedBuilder::default();
    embed
        .title("It's fucked")
        .colour(crate::COLOUR_DANGER)
        .description(format!("```rs\n{}```", error))
        .footer(|footer| footer.text("Okay, Houston, I believe we've had a problem here ..."));
    embed
}
