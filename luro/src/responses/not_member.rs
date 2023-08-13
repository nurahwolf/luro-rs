use tracing::warn;
use twilight_util::builder::embed::EmbedBuilder;

use crate::COLOUR_DANGER;

use crate::interaction::LuroSlash;

impl LuroSlash {
    pub async fn not_member_response(&self, username: &String) -> anyhow::Result<()> {
        self.respond(|r| r.add_embed(not_member_embed(username).build())).await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn not_member_embed(username: &String) -> EmbedBuilder {
    warn!("User is no longer a member of the server");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .description(format!("I'm afraid {username} is no longer a member of the server."))
}
