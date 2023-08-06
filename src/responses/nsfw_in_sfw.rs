use tracing::info;

use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::COLOUR_DANGER;

use crate::framework::LuroFramework;
use crate::models::LuroResponse;

impl LuroFramework {
    pub async fn nsfw_in_sfw_response(&self, slash: &mut LuroResponse) -> anyhow::Result<()> {
        slash.embed(nsfw_in_sfw_embed().build())?;
        self.respond(slash).await
    }
}

/// Returns an embed containing a standardised error that the user is running a NSFW command in a SFW channel
fn nsfw_in_sfw_embed() -> EmbedBuilder {
    info!("Attempting to run a naughty command in a safe for wah channel!");
    EmbedBuilder::new()
        .title("This is a Safe For Wah (SFW) channel!")
        .color(COLOUR_DANGER)
        .description("This is a NAUGHTY command. That means no doing this where minors could be **>:C**")
        .footer(EmbedFooterBuilder::new("Do it again and I'm reporting you to the FBI."))
}
