use luro_model::builders::EmbedBuilder;

use crate::models::interaction::InteractionError;

pub fn internal_error(error: &InteractionError) -> EmbedBuilder {
    let mut embed = EmbedBuilder::default();
    embed
        .title("It's fucked")
        .colour(crate::COLOUR_DANGER)
        .description(format!("```rs\n{}```", error))
        .footer(|footer| footer.text("Okay, Houston, I believe we've had a problem here ..."));
    embed
}
