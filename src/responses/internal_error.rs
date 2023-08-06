use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::COLOUR_DANGER;

use crate::framework::LuroFramework;
use crate::models::LuroResponse;

impl LuroFramework {
    /// A response returned by default when a command does not exist within Luro.
    pub async fn internal_error_response(&self, error: String, slash: &mut LuroResponse) -> anyhow::Result<()> {
        // TODO: Test for deferred
        slash.embed(Self::internal_error_embed(error.clone()).build())?.ephemeral();
        self.respond(slash).await
    }

    pub fn internal_error_embed(error: String) -> EmbedBuilder {
        EmbedBuilder::new()
            .title("It's fucked")
            .color(COLOUR_DANGER)
            .description(format!("```{error}```"))
            .footer(EmbedFooterBuilder::new(
                "Okay, Houston, I believe we've had a problem here ..."
            ))
    }
}
