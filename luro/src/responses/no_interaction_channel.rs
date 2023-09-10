use luro_model::database::drivers::LuroDatabaseDriver;
use luro_model::response::LuroResponse;
use tracing::error;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::COLOUR_DANGER;

use crate::interaction::LuroSlash;

impl<D: LuroDatabaseDriver> LuroSlash<D> {
    pub async fn no_interaction_channel_response(&self) -> anyhow::Result<()> {
        self.respond(|r: &mut LuroResponse| r.add_embed(no_interaction_channel_embed().build()))
            .await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn no_interaction_channel_embed() -> EmbedBuilder {
    error!("Unable to get the interaction channel");
    EmbedBuilder::new()
        .title("Unable to get interaction channel")
        .color(COLOUR_DANGER)
        .description("I'm afraid I was unable to work out what channel this command was ran in. Try again as it might be the API that's having a moment.")
        .footer(EmbedFooterBuilder::new(
            "Okay, Houston, I believe we've had a problem here ...",
        ))
}
