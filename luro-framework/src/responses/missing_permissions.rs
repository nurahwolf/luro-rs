use luro_builder::embed::EmbedBuilder;
use luro_model::COLOUR_DANGER;
use tracing::error;
use twilight_model::guild::Permissions;

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
pub fn missing_permission_embed(permission: Permissions) -> EmbedBuilder {
    error!("User was missing permissions to run a command");
    let mut embed = EmbedBuilder::default();
    embed
        .colour(COLOUR_DANGER)
        .title("You are missing permissions")
        .description(format!("Hey dork! You need {:?} permission to run this command.", permission));
    embed
}
