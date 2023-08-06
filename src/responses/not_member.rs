use tracing::warn;

use twilight_util::builder::embed::EmbedBuilder;

use crate::COLOUR_DANGER;

use crate::framework::LuroFramework;
use crate::models::LuroResponse;

impl LuroFramework {
    pub async fn not_member_response(&self, username: &String, slash: &mut LuroResponse) -> anyhow::Result<()> {
        slash.embed(not_member_embed(username).build())?;
        self.respond(slash).await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn not_member_embed(username: &String) -> EmbedBuilder {
    warn!("User is no longer a member of the server");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .description(format!("I'm afraid {username} is no longer a member of the server."))
}
