/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
pub fn missing_permission_embed(permission: &twilight_model::guild::Permissions) -> crate::builders::EmbedBuilder {
    tracing::error!("User was missing permissions to run a command");
    let mut embed = crate::builders::EmbedBuilder::default();
    embed
        .colour(crate::COLOUR_DANGER)
        .title("You are missing permissions")
        .description(format!("Hey dork! You need {:?} permission to run this command.", permission));
    embed
}
