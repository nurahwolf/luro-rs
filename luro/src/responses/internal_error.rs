use anyhow::Error;
use luro_builder::embed::EmbedBuilder;

use crate::COLOUR_DANGER;

use crate::interaction::LuroSlash;

impl LuroSlash {
    /// A response returned by default when a command does not exist within Luro.
    pub async fn internal_error_response(&self, error: Error) -> anyhow::Result<()> {
        self.respond(|respond| respond.add_embed(internal_error_embed(error)).ephemeral())
            .await
    }
}

fn internal_error_embed(error: Error) -> EmbedBuilder {
    let mut embed = EmbedBuilder::default();
    embed
        .title("It's fucked")
        .colour(COLOUR_DANGER)
        .description(format!("```{:#?}```", error.to_string()))
        .footer(|footer| footer.text("Okay, Houston, I believe we've had a problem here ..."));
    embed
}
