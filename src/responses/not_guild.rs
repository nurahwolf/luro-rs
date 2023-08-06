use tracing::warn;

use twilight_util::builder::embed::EmbedBuilder;

use crate::COLOUR_DANGER;

use crate::framework::LuroFramework;
use crate::models::LuroResponse;

impl LuroFramework {
    pub async fn not_guild_response(&self, slash: &mut LuroResponse) -> anyhow::Result<()> {
        slash.embed(not_guild_embed().build())?;
        self.respond(slash).await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn not_guild_embed() -> EmbedBuilder {
    warn!("Guild Command was ran but I could not work out what guild they were in");
    EmbedBuilder::new()
        .title("Unable to find this guild!")
        .color(COLOUR_DANGER)
        .description("If you ran this command in a guild, I am unable to find it. If this is a DM... Please tell Nurah to limit this command to guilds only.")
}
