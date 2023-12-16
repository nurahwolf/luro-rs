use crate::types::User;

/// An embed returned if the user is above the bot in the role hierarchy.
pub fn bot_hierarchy_embed(user: &User, bot: &User) -> crate::builders::EmbedBuilder {
    tracing::warn!("User tried to execute a command in which the bot is too low to function");
    let mut embed = crate::builders::EmbedBuilder::default();
    let reason = "You are trying to perform an action on someone higher than me!\nDiscord will not let you perform this action, sorry!".to_string();

    embed.colour(crate::COLOUR_DANGER)
        .title("Role Hierarchy Error")
        .description(reason);

    if let Some(member) = &user.member && let Some(member_data) = &member.data && let Some(highest_role) = member_data.highest_role() {
        embed.create_field(&user.name(), &format!("{} - `{}`", highest_role.name, highest_role.position), true);
    }

    if let Some(member) = &bot.member && let Some(member_data) = &member.data && let Some(highest_role) = member_data.highest_role() {
        embed.create_field(&user.name(), &format!("{} - `{}`", highest_role.name, highest_role.position), true);
    }

    embed
}
