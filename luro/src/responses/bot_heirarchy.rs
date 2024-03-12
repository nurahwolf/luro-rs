use crate::{builders::EmbedBuilder, models::MemberContext};

/// An embed returned if the user is above the bot in the role hierarchy.
pub fn bot_hierarchy_embed(user: &mut MemberContext, bot: &mut MemberContext) -> EmbedBuilder {
    tracing::warn!(
        "`{}` tried to execute a command in which the bot is too low to function",
        user.username()
    );

    let mut embed = EmbedBuilder::default();
    let user_name = user.username();
    let bot_name = bot.username();

    embed
        .colour(crate::COLOUR_DANGER)
        .title("⚠️ Role Hierarchy Error ⚠️")
        .description("You are trying to perform an action on someone higher than me!\nDiscord will not let you perform this action, sorry!");

    if let Some(user_role) = user.highest_role() {
        embed.create_field(
            user_name,
            format!("{} - `{}`", user_role.role.name, user_role.role.position),
            true,
        );
    }

    if let Some(bot_role) = bot.highest_role() {
        embed.create_field(
            bot_name,
            format!("{} - `{}`", bot_role.role.name, bot_role.role.position),
            true,
        );
    }

    embed
}
