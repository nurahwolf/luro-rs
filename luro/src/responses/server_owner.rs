use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::warn;
use twilight_util::builder::embed::EmbedBuilder;

use crate::COLOUR_DANGER;

use crate::interaction::LuroSlash;

impl<D: LuroDatabaseDriver> LuroSlash<D> {
    pub async fn server_owner_response(&self) -> anyhow::Result<()> {
        self.respond(|r| r.add_embed(server_owner_embed().build())).await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn server_owner_embed() -> EmbedBuilder {
    warn!("Someone tried to fuck with the server owner using the bot");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("That's the server owner you idiot")
        .description("Congratulations moron, that's the server owner. Do you really think I'm gonna try to kick OR ban them? Holy shit, no.")
}
