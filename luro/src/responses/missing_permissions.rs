use luro_model::database_driver::LuroDatabaseDriver;
use tracing::error;
use twilight_model::guild::Permissions;
use twilight_util::builder::embed::EmbedBuilder;

use crate::COLOUR_DANGER;

use crate::interaction::LuroSlash;

impl<D: LuroDatabaseDriver> LuroSlash<D> {
    pub async fn missing_permission_response(&self, permission: Permissions) -> anyhow::Result<()> {
        self.respond(|r| r.add_embed(missing_permission_embed(permission).build()))
            .await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn missing_permission_embed(permission: Permissions) -> EmbedBuilder {
    error!("User was missing permissions to run a command");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("You are missing permissions")
        .description(format!("Hey dork! You need {:?} permission to run this command.", permission))
}
