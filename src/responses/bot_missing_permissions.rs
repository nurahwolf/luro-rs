use tracing::error;
use twilight_util::builder::embed::EmbedBuilder;

use crate::COLOUR_DANGER;

use crate::models::LuroSlash;

impl LuroSlash {
    pub async fn bot_missing_permission_response(self, permission_missing: &String) -> anyhow::Result<()> {
        self.embed(bot_missing_permission_embed(permission_missing).build())?
            .respond()
            .await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn bot_missing_permission_embed(permission_missing: &String) -> EmbedBuilder {
    error!("Luro was missing permissions to run a command");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("I am missing permissions")
        .description(format!("***SOME*** motherfucker failed to set me up correctly.\nI should have ***Administrator*** privileges in the server to work my best, but it seems I'm missing that. Fix it >:c\nIf you explicitly want to limit my permissions, I'm missing the {permission_missing} permisison for this command to work."))
}
