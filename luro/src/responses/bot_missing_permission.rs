/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
pub fn bot_missing_permission_embed(permission: &twilight_model::guild::Permissions) -> luro_model::builders::EmbedBuilder {
    tracing::error!("Luro was missing permissions to run a command");
    let mut embed = luro_model::builders::EmbedBuilder::default();
    embed.colour(crate::COLOUR_DANGER)
        .title("I am missing permissions")
        .description(format!("***SOME*** motherfucker failed to set me up correctly.\nI should have ***Administrator*** privileges in the server to work my best, but it seems I'm missing that. Fix it >:c\nIf you explicitly want to limit my permissions, I'm missing the {:?} permission for this command to work.", permission));
    embed
}
