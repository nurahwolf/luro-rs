use luro_model::builders::EmbedBuilder;

/// A response for if someone tried to screw with a server owner
pub fn permission_server_owner(user_id: &twilight_model::id::Id<twilight_model::id::marker::UserMarker>) -> EmbedBuilder {
    tracing::warn!("User {user_id} attempted to abuse a server owner");
    let mut embed = EmbedBuilder::default();
    embed
        .colour(crate::COLOUR_DANGER)
        .title("That's the server owner you idiot")
        .description(format!(
            "Congratulations <@{user_id}>, that's the server owner. Do you really think I'm gonna screw with them? Nope!"
        ));
    embed
}
