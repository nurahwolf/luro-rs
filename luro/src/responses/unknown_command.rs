use tracing::error;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::COLOUR_DANGER;

use crate::slash::Slash;

impl Slash {
    /// A response sent when Luro receives a command it does not have a handler for
    pub async fn unknown_command_response(mut self) -> anyhow::Result<()> {
        self.embed(unknown_command_embed().build())?.respond().await
    }
}

/// Unknown command received embed
fn unknown_command_embed() -> EmbedBuilder {
    error!("Unknown command received, most likely its not registered in the event handler. You want to fix this.");
    EmbedBuilder::new()
        .title("Unknown Command Received")
        .color(COLOUR_DANGER)
        .description("This command does not exist yet, sorry!")
        .footer(EmbedFooterBuilder::new("We had a fucky wucky!"))
}
