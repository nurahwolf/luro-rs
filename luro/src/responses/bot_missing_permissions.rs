use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::error;
use twilight_model::guild::Permissions;
use twilight_util::builder::embed::EmbedBuilder;

use crate::COLOUR_DANGER;

use crate::interaction::LuroSlash;

impl<D: LuroDatabaseDriver> LuroSlash<D> {
    pub async fn bot_missing_permission_response(&self, permission: Permissions) -> anyhow::Result<()> {
        self.respond(|r| r.add_embed(bot_missing_permission_embed(permission).build()))
            .await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn bot_missing_permission_embed(permission: Permissions) -> EmbedBuilder {
    error!("Luro was missing permissions to run a command");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("I am missing permissions")
        .description(format!("***SOME*** motherfucker failed to set me up correctly.\nI should have ***Administrator*** privileges in the server to work my best, but it seems I'm missing that. Fix it >:c\nIf you explicitly want to limit my permissions, I'm missing the {:?} permission for this command to work.", permission))
}
