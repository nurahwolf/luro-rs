/// An embed returned if the user is above the bot in the role hierarchy.
pub fn bot_hierarchy_embed(bot_username: &str) -> crate::builders::EmbedBuilder {
    tracing::warn!("User tried to execute a command in which the bot is too low to function");
    let mut embed = crate::builders::EmbedBuilder::default();
    embed.colour(crate::COLOUR_DANGER)
        .title("Role Hierarchy Error")
        .description(format!("This member has a role above or equivalent to that of {bot_username} in the list of roles, which prevents moderation actions from being performed on them. You can correct this by placing me higher in the list of roles.\nHowever, if you are trying to action someone higher than me on purpose... ***Get fucked ;)***"));
    embed
}