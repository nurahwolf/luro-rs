use crate::types::User;

/// An embed returned if the user action is above the user executing in the role hierarchy.
pub fn user_hierarchy_embed(user: &User, target: &User) -> crate::builders::EmbedBuilder {
    tracing::warn!("User tried to execute a command in which the bot is too low to function");
    let mut embed = crate::builders::EmbedBuilder::default();
    let reason = "You are trying to perform an action on someone higher than you!\nI'm not going to let you have free privilege escalation...".to_string();

    embed.colour(crate::COLOUR_DANGER)
        .title("Role Hierarchy Error")
        .description(reason);

    if let Some(member) = &user.member && let Some(member_data) = &member.data && let Some(highest_role) = member_data.highest_role() {
        embed.create_field(&user.name(), &format!("{} - `{}`", highest_role.name, highest_role.position), true);
    }

    if let Some(member) = &target.member && let Some(member_data) = &member.data && let Some(highest_role) = member_data.highest_role() {
        embed.create_field(&user.name(), &format!("{} - `{}`", highest_role.name, highest_role.position), true);
    }

    embed
}