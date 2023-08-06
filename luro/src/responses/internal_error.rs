use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::COLOUR_DANGER;

use crate::models::LuroSlash;

impl LuroSlash {
    /// A response returned by default when a command does not exist within Luro.
    pub async fn internal_error_response(&mut self, error: String) -> anyhow::Result<()> {
        if (self
            .embed(internal_error_embed(error.clone()).build())?
            .ephemeral()
            .respond()
            .await)
            .is_err()
        {
            self.set_deferred().respond().await
        } else {
            Ok(())
        }
    }
}

fn internal_error_embed(error: String) -> EmbedBuilder {
    EmbedBuilder::new()
        .title("It's fucked")
        .color(COLOUR_DANGER)
        .description(format!("```{error}```"))
        .footer(EmbedFooterBuilder::new(
            "Okay, Houston, I believe we've had a problem here ..."
        ))
}
