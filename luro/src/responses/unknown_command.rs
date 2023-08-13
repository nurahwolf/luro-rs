use luro_builder::embed::EmbedBuilder;
use tracing::error;

use crate::COLOUR_DANGER;

use crate::interaction::LuroSlash;
use crate::slash::Slash;

impl Slash {
    /// A response sent when Luro receives a command it does not have a handler for
    pub async fn unknown_command_response(mut self) -> anyhow::Result<()> {
        self.embed(unknown_command_embed().into())?.respond().await
    }
}

impl LuroSlash {
    /// A response sent when Luro receives a command it does not have a handler for
    pub async fn unknown_command_response(self) -> anyhow::Result<()> {
        self.respond(|r| r.add_embed(unknown_command_embed())).await
    }
}

/// Unknown command received embed
fn unknown_command_embed() -> EmbedBuilder {
    error!("Unknown command received, most likely its not registered in the event handler. You want to fix this.");
    let mut embed = EmbedBuilder::default();
    embed
        .title("Unknown Command Received")
        .colour(COLOUR_DANGER)
        .description("This command does not exist yet, sorry!")
        .footer(|footer| footer.text("We had a fucky wucky!"));
    embed
}
