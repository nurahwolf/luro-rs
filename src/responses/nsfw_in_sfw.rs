use tracing::warn;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::COLOUR_DANGER;

use super::LuroSlash;

impl LuroSlash {
    pub async fn nsfw_in_sfw_response(self) -> anyhow::Result<()> {
        self.embed(nsfw_in_sfw_embed().build())?.respond().await
    }
}

/// Returns an embed containing a standardised error that the user is running a NSFW command in a SFW channel
fn nsfw_in_sfw_embed() -> EmbedBuilder {
    // TODO: Tweak this
    warn!("Someone tried to run a bot owner command without being the bot owner...r");
    EmbedBuilder::new()
    .title("You are not the bot owner!")
    .color(COLOUR_DANGER)
    .description("Great job motherfucker, you are not the bot owner and do not have permission to use that command.\n\n**THE COMMAND IS LITERALLY NAMED OWNER ONLY! WHAT THE HECK DID YOU THINK WOULD HAPPEN!?**")
    .footer(EmbedFooterBuilder::new("FYI, I'm reporting you to Nurah."))
}
